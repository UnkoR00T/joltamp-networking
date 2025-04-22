use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    id: Option<Thing>,
    #[serde(rename = "firstname")]
    first_name: String,
    #[serde(rename = "lastname")]
    last_name: String,
    email: String,
    password: String,
    jwt: Option<Uuid>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountSafe {
    id: Option<Thing>,
    #[serde(rename = "firstname")]
    first_name: String,
    #[serde(rename = "lastname")]
    last_name: String,
    email: String
}

#[derive(Serialize, Deserialize)]
pub struct AccountResponse{
    pub id: Id,
    jwt: Uuid
}

impl From<Account> for AccountResponse{
    fn from(acc: Account) -> Self {
        AccountResponse {
                id: acc.id.unwrap().id,
                jwt: acc.jwt.unwrap()
            }
    }
}