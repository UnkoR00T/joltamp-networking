use rocket::http::Status;
use rocket::post;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use crate::DB;
use crate::db::error::error::Error;
use crate::guards::auth_guard::AuthToken;
use crate::guards::networking_admin_guard::AdminGuard;
use crate::routes::auth_account::auth_account;
use crate::routes::get_app::get_app;
use crate::services::check_app_existence::check_app_existence;
use crate::types::app::{App, CreateAppRequest, CreateAppResponse};

#[derive(Serialize, Deserialize)]
struct ChangePermsRequest {
    app_id: String,
    perms: Vec<String>
}

#[post("/app/change_perms", data = "<app>")]
pub async fn change_app_perms(app: Json<ChangePermsRequest>, admin: AdminGuard) -> Result<Status, Error> {
    let app_id = app.app_id.clone();
    let perms = app.perms.clone();
    match check_app_existence(&app_id).await{
        Ok(_) => {
            let query = r#"
                UPDATE $app SET perms = $perms;
            "#;
            let query = DB.query(query)
                .bind(("app", Thing::from(("auth_apps", Id::String(app_id)))))
                .bind(("perms", perms))
                .await?;
            Ok(Status::Ok)
        },
        Err(e) => Err(Error::Custom(Status::BadRequest))
    }
}