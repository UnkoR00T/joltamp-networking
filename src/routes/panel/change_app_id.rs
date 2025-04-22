use rocket::http::Status;
use rocket::post;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::sql::{Id, Thing};
use crate::DB;
use crate::db::error::error::Error;
use crate::guards::networking_admin_guard::AdminGuard;
use crate::routes::get_app::get_app;
use crate::services::check_app_existence::check_app_existence;

#[derive(Serialize, Deserialize)]
struct ChangePermsRequest {
    cur_app_id: String,
    new_app_id: String
}

#[post("/app/change_id", data = "<app>")]
pub async fn change_app_id(app: Json<ChangePermsRequest>, admin: AdminGuard) -> Result<Status, Error> {
    let cur_app_id = app.cur_app_id.clone();
    let new_app_id = app.new_app_id.clone();
    if cur_app_id == "Networking" {
        return Err(Error::Custom(Status::NotAcceptable));
    }
    match check_app_existence(&cur_app_id).await{
        Ok(_) => {
            if let Ok(_) = check_app_existence(&new_app_id).await{
                return Err(Error::Custom(Status::NotAcceptable));
            }
            let query = r#"
                LET $app_info = (SELECT perms, url FROM $cur_app)[0];
                CREATE $new_app CONTENT {
                    name: $new_app_id,
                    perms: $app_info.perms,
                    url: $app_info.url
                };
                UPDATE auths SET out = $new_app WHERE out == $cur_app;
                DELETE $cur_app;
            "#;
            let query = DB.query(query)
                .bind(("cur_app", Thing::from(("auth_apps", Id::String(cur_app_id)))))
                .bind(("new_app", Thing::from(("auth_apps", Id::String(new_app_id.clone())))))
                .bind(("new_app_id", new_app_id))
                .await?;
            Ok(Status::Ok)
        },
        Err(e) => Err(Error::Custom(Status::BadRequest))
    }
}