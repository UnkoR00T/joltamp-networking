use rocket::{post, uri, Response};
use rocket::http::{ContentType, Status};
use rocket::response::{content, status, Redirect, Responder};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use uuid::Uuid;
use crate::DB;
use crate::db::error::error::Error;
use crate::routes::auth_account::auth_account;
use crate::routes::get_app::get_app;
use crate::routes::verify_account::verify_account;
use crate::types::account::{Account, AccountResponse};
use crate::types::app::CreateAppRequest;
use crate::guards::auth_guard::{AuthToken};

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthRequest{
    pub authorize: bool
}

#[derive(Serialize,Deserialize, Clone)]
pub struct Auths{
    #[serde(rename="in")]
    pub in_value: Thing,
    #[serde(rename="out")]
    pub out_value: Thing
}

#[post("/app/auth?<app>", data = "<auth>")]
pub async fn authorize_app(app: &str, auth: Json<AuthRequest>, auth_token: AuthToken) -> Result<Status, Error> {
    let auth: AuthRequest = auth.0;
    match get_app(app, auth_token.clone()).await {
        Ok(app_data) => {
            match auth_account(app, auth_token.clone()).await {
                Ok(_) => {
                    Ok(Status::Ok)
                },
                Err(_) => {
                    if(!auth.authorize){
                        Ok(Status::Accepted)
                    }else{
                        let app_data: CreateAppRequest = app_data.1.0;
                        let auth_record: Auths = Auths{
                            in_value: Thing::from(("account", verify_account(auth_token).await?.1.0)),
                            out_value: Thing::from(("auth_apps", Id::String(app_data.name.to_string())))
                        };
                        let record: Option<Auths> = DB
                            .create("auths")
                            .content(Json(auth_record).into_inner())
                            .await?;
                        match record {
                            Some(_) => {
                                Ok(Status::Ok)
                            },
                            None => {
                                Err(Error::Custom(Status::InternalServerError))
                            }
                        }
                    }
                }
            }
        },
        Err(e) => {
            Err(e)
        }
    }
}