use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::routes::auth_account::auth_account;
use crate::guards::auth_guard::AuthToken;

pub struct AdminGuard(pub AuthToken);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminGuard {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            let auth_token = AuthToken(auth_header.to_string());
            match auth_account("Networking", auth_token.clone()).await {
                Ok(admin) => {
                    if admin.1.0 {
                        Outcome::Success(AdminGuard(auth_token))
                    }else{
                        Outcome::Error((Status::Unauthorized, String::from("Networking admin is not authorized correctly")))
                    }
                },
                Err(_e) => {
                    Outcome::Error((Status::BadRequest, String::from("Failed to authenticate user in Networking")))
                }
            }
        } else {
            Outcome::Error((Status::BadRequest, String::from("Missing Authorization Header")))
        }
    }
}