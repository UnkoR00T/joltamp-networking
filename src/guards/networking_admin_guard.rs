use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::routes::auth_account::auth_account;
use crate::guards::auth_guard::AuthToken;

pub struct AdminGuard(pub bool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminGuard {
    type Error = Status;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            match auth_account("Networking", AuthToken(auth_header.to_string())).await {
                Ok(admin) => {
                    if(admin.1.0){
                        Outcome::Success(AdminGuard(true))
                    }else{
                        Outcome::Error((Status::Unauthorized, Status::Unauthorized))
                    }
                },
                Err(_e) => {
                    Outcome::Error((Status::BadRequest, Status::BadRequest))
                }
            }
        } else {
            Outcome::Error((Status::BadRequest, Status::BadRequest))
        }
    }
}