use rocket::{get, Response};
use rocket::http::{ContentType, Status};
use rocket::response::{content, status, Responder};
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use surrealdb::sql::{Id, Thing};
use uuid::Uuid;
use crate::DB;
use crate::db::error::error::Error;
use crate::routes::auth_account::auth_account;
use crate::routes::verify_account::verify_account;
use crate::types::account::{AccountRequest, AccountResponse};
use crate::types::app::CreateAppRequest;
use crate::types::auth::AuthToken;
use crate::types::login::LoginQuery;

#[get("/app?<app>")]
pub async fn get_app(app: &str, auth_token: AuthToken) -> Result<status::Custom<Json<CreateAppRequest>>, Error> {
    match verify_account(auth_token).await {
        Ok(_) => {
            let query = r#"
                SELECT name, perms, url FROM $app;
            "#;
            let mut q_res = DB
                .query(query)
                .bind(("app", Thing::from(("auth_apps", Id::String(app.to_string())))))
                .await?;
            let app_info: Option<CreateAppRequest> = q_res.take(0)?;
            match app_info {
                Some(app_info) => {
                    Ok(status::Custom(Status::Ok, Json(app_info)))
                },
                None => {
                    Err(Error::Custom(Status::NotFound))
                }
            }
        },
        Err(e) => {
            Err(e)
        }
    }
}