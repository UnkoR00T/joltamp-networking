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
use crate::types::account::{Account, AccountResponse};
use crate::guards::auth_guard::{AuthToken};
use surrealdb::sql::Uuid;
use crate::services::is_admin_account::is_admin_account;

#[post("/account/auth?<app>")]
pub async fn auth_account(app: &str, auth_token: AuthToken) -> Result<status::Custom<Json<bool>>, Error> {
    let jwt = Uuid::from_str(&*auth_token.0);
    match jwt {
        Ok(jwt) => {
            let admin = is_admin_account(jwt, app).await?;
            Ok(status::Custom(Status::Ok, Json(admin)))
        },
        _ => {
            Err(Error::Custom(Status::BadRequest))
        }
    }

}