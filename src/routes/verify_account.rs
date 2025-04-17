use std::str::FromStr;
use rocket::http::Status;
use rocket::post;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing, Uuid};
use crate::DB;
use crate::db::error::error::Error;
use crate::types::auth::AuthToken;
use crate::types::login::LoginQuery;

#[derive(Serialize, Deserialize)]
struct QueryResult {
    admin: bool,
}

#[derive(Serialize, Deserialize)]
struct AccountIdResult {
    id: Thing,
}

#[post("/account/verify")]
pub async fn verify_account(auth_token: AuthToken) -> Result<status::Custom<Json<Id>>, Error> {
    let auth_token: Result<Uuid, _> = Uuid::from_str(&*auth_token.0);
    let auth_token = auth_token.unwrap_or_else(|_| Uuid::new());
    let jwt_lookup_query = r#"
        SELECT id FROM account WHERE jwt = $jwt;
    "#;
    let mut id_query_res = DB.query(jwt_lookup_query)
        .bind(("jwt", auth_token))
        .await?;
    let account_result: Option<AccountIdResult> = id_query_res.take(0)?;
    match account_result {
        Some(data) => Ok(status::Custom(Status::Ok, Json(data.id.id))),
        None => return Err(Error::Custom(Status::NotFound)),
    }
}
