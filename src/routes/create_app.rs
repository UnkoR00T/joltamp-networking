use rocket::{post, Response};
use rocket::http::{ContentType, Status};
use rocket::response::{content, status, Responder};
use rocket::serde::json::Json;
use uuid::Uuid;
use crate::DB;
use crate::db::error::error::Error;
use crate::routes::auth_account::auth_account;
use crate::types::account::{Account, AccountResponse};
use crate::types::app::{App, CreateAppRequest, CreateAppResponse};
use crate::guards::auth_guard::AuthToken;

#[post("/app/create", data = "<app>")]
pub async fn create_app(app: Json<CreateAppRequest>, auth_token: AuthToken) -> Result<status::Custom<Json<CreateAppResponse>>, Error> {
    match auth_account("Networking", auth_token).await {
        Ok(status) => {
            let admin: bool = status.1.0;
            if(admin == false){
                return Err(Error::Custom(Status::Unauthorized));
            }
            let app_data: &CreateAppRequest = &app.0;
            let res: Option<App> = DB.create(("auth_apps", &app_data.name))
                .content(app.into_inner())
                .await?;
            match res {
                Some(res) => Ok(status::Custom(Status::Ok, Json(CreateAppResponse::from(res)))),
                None => Err(Error::Db)
            }
        }
        Err(_) => {
            Err(Error::Custom(Status::Unauthorized))
        }
    }
}