use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::DB;
use crate::db::error::error::Error;
use crate::routes::auth_account::auth_account;
use crate::types::app::{App, CreateAppRequest};
use crate::guards::auth_guard::AuthToken;
use crate::guards::networking_admin_guard::AdminGuard;

#[get("/apps")]
pub async fn get_apps(admin: AdminGuard) -> Result<status::Custom<Json<Vec<App>>>, Error> {
    if(admin.0) {
        let query = r#"
                    SELECT * FROM auth_apps;
                "#;
        let mut q_res = DB
            .query(query)
            .await?;
        let apps: Vec<App> = q_res.take(0)?;
        Ok(status::Custom(Status::Ok, Json(apps)))
    }else{
        Err(Error::Custom(Status::Unauthorized))
    }
}