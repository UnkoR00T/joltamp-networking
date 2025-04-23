use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use crate::DB;
use crate::db::error::error::Error;
use crate::guards::networking_admin_guard::AdminGuard;
use crate::services::check_app_existence::check_app_existence;

#[derive(Serialize, Deserialize)]
struct RemoveAppRequest {
    app_id: String
}

/// Remove app route handler function ->
/// Status:
/// - 200: Ok
/// - 400: BadRequest (App Notfound)
/// - 406: NotAcceptable (Cant delete Networking app)
#[post("/app/remove", data = "<app>")]
pub async fn rm_app(app: Json<RemoveAppRequest>, _admin: AdminGuard) -> Result<Status, Error> {
    let app_id = app.app_id.clone();
    if app_id == "Networking" {
        return Err(Error::Custom(Status::NotAcceptable));
    }
    match check_app_existence(&app_id).await{
        Ok(_) => {
            let query = r#"
                DELETE auths WHERE out == $cur_app;
                DELETE $cur_app;
            "#;
            DB.query(query)
                .bind(("cur_app", Thing::from(("auth_apps", Id::String(app_id)))))
                .await?;
            Ok(Status::Ok)
        },
        Err(_e) => Err(Error::Custom(Status::BadRequest))
    }
}