use std::str::FromStr;
use rocket::{post, Response};
use rocket::http::{ContentType, Status};
use rocket::log::private::info;
use rocket::outcome::Outcome;
use rocket::response::{content, status, Responder};
use rocket::serde::json::Json;
// use uuid::Uuid;
use crate::DB;
use crate::db::error::error::Error;
use crate::types::account::{AccountRequest, AccountResponse};
use crate::types::auth::{AuthToken};
use surrealdb::sql::Uuid;

#[post("/account/auth/<app>")]
pub async fn auth_account(app: &str, auth_token: AuthToken) -> Result<Status, Error> {
    let app = Uuid::from_str(app);
    match app {
        Ok(app) => {
            info!("App: {app}, Token: {0}", auth_token.0);
            Ok(Status::Ok)
        },
        Err(_) => {
            Err(Error::Db)
        }
    }

}