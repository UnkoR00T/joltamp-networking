use std::str::FromStr;
use rocket::http::Status;
use rocket::response::status;
use surrealdb::sql::{Id, Thing, Uuid};
use crate::DB;
use crate::db::error::error::Error;
use crate::types::account::Account;

pub async fn check_user_existence(user: &str) -> Result<status::Custom<Account>, Error> {
    let query = r#"
        SELECT * FROM $user;
    "#;
    let user_id = Uuid::from_str(&user);
    let user_id = user_id.unwrap_or_else(|_| Uuid::new());
    let user_thing = Thing::from(("account".to_string(), Id::Uuid(user_id)));
    let mut q_res = DB.query(query)
        .bind(("user", user_thing))
        .await?;
    let is_admin: Option<Account> = q_res.take(0)?;
    match is_admin {
        Some(account) => {
            Ok(status::Custom(Status::Ok, account))
        },
        None => {
            Err(Error::Custom(Status::NotFound))
        }
    }
}