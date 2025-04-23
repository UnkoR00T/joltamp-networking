use std::str::FromStr;
use rocket::http::Status;
use rocket::post;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use surrealdb::sql::{Id, Thing, Uuid};
use crate::DB;
use crate::db::error::error::Error;
use crate::guards::networking_admin_guard::AdminGuard;
use crate::routes::get_app::get_app;
use crate::services::check_user_existence::check_user_existence;

#[derive(Serialize, Deserialize)]
struct ChangeJWTRequest {
    user_id: String,
}

/// Change user JWT route handler function for changing user jwt on demand.
#[post("/user/change_jwt", data = "<app>")]
pub async fn change_user_jwt(app: Json<ChangeJWTRequest>, _admin: AdminGuard) -> Result<Status, Error> {
    let user_id = app.user_id.clone();
    match check_user_existence(&user_id).await{
        Ok(_) => {
            let user_id = Uuid::from_str(&user_id);
            let user_id = user_id.unwrap_or_else(|_| Uuid::new());
            let query = r#"
                UPDATE $user SET jwt = rand::uuid::v7();
            "#;
            DB.query(query)
                .bind(("user", Thing::from(("account", Id::Uuid(user_id)))))
                .await?;
            Ok(Status::Ok)
        },
        Err(_e) => Err(Error::Custom(Status::BadRequest))
    }
}