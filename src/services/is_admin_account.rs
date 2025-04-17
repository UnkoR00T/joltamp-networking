use std::str::FromStr;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing, Uuid};
use crate::DB;
use crate::db::error::error::Error;
use crate::services::check_app_existence::check_app_existence;

#[derive(Serialize, Deserialize)]
struct QueryResult {
    admin: bool,
}

#[derive(Serialize, Deserialize)]
struct AccountIdResult {
    id: Thing,
}

pub async fn is_admin_account(jwt: Uuid, app: &str) -> Result<bool, Error> {
    check_app_existence(app).await?;
    let jwt_lookup_query = r#"
        SELECT id FROM account WHERE jwt = $jwt;
    "#;
    let mut id_query_res = DB.query(jwt_lookup_query)
        .bind(("jwt", jwt))
        .await?;
    let account_result: Option<AccountIdResult> = id_query_res.take(0)?;
    let account_id = match account_result {
        Some(data) => data.id,
        None => return Err(Error::Custom(Status::NotFound)),
    };
    let app_thing = Thing::from(("auth_apps".to_string(), Id::String(app.to_string())));
    let admin_query = r#"
        SELECT admin FROM auths
        WHERE in = $account_id AND out = $app;
    "#;
    let mut admin_check_res = DB.query(admin_query)
        .bind(("account_id", account_id))
        .bind(("app", app_thing))
        .await?;
    let is_admin: Option<QueryResult> = admin_check_res.take(0)?;
    match is_admin {
        Some(res) => Ok(res.admin),
        None => Err(Error::Custom(Status::Continue)),
    }
}
