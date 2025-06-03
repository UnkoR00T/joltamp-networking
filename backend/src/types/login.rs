use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest{
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginQuery{
    pub id: Thing,
    pub password: String,
    jwt: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse{
    pub id: Id,
    jwt: Uuid
}

impl From<LoginQuery> for LoginResponse{
    fn from(acc: LoginQuery) -> Self {
        LoginResponse {
            id: acc.id.id,
            jwt: acc.jwt
        }
    }
}