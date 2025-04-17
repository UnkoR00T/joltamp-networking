use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};

#[derive(Debug)]
pub struct AuthToken(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            Some(token) => Outcome::Success(AuthToken(token.to_string())),
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
