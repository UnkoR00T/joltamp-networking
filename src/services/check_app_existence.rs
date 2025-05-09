use rocket::http::Status;
use rocket::response::status;
use surrealdb::sql::{Id, Thing};
use crate::DB;
use crate::db::error::error::Error;
use crate::types::app::App;

pub async fn check_app_existence(app: &str) -> Result<status::Custom<App>, Error> {
    let query = r#"
        SELECT * FROM $app;
    "#;
    let app_thing = Thing::from(("auth_apps".to_string(), Id::String(app.to_string())));
    let mut q_res = DB.query(query)
        .bind(("app", app_thing))
        .await?;
    let is_admin: Option<App> = q_res.take(0)?;
    match is_admin {
        Some(app) => {
            Ok(status::Custom(Status::Ok, app))
        },
        None => {
            Err(Error::Custom(Status::NotFound))
        }
    }
}