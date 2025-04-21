use rocket::{post, Response};
use rocket::http::{ContentType, Status};
use rocket::response::{content, status, Responder};
use rocket::serde::json::Json;
use uuid::Uuid;
use crate::DB;
use crate::db::error::error::Error;
use crate::types::account::{Account, AccountResponse};

#[post("/account/register", data = "<account>")]
pub async fn create_account(account: Json<Account>) -> Result<status::Custom<Json<AccountResponse>>, Error> {
    let account: Option<Account> = DB
        .create(("account", Uuid::now_v7()))
        .content(account.into_inner())
        .await?;
    match account {
        Some(acc) => {
            Ok(status::Custom(Status::Created,
                Json(AccountResponse::from(acc))))
        },
        None => Err(Error::Db),
    }
}