use std::str::FromStr;
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing, Uuid};
use crate::DB;
use crate::db::error::error::Error;
use crate::guards::networking_admin_guard::AdminGuard;
use crate::services::check_app_existence::check_app_existence;

#[derive(Serialize, Deserialize)]
struct RemoveUserRequest {
    user_id: String
}

#[post("/user/remove", data = "<user>")]
pub async fn rm_user(user: Json<RemoveUserRequest>, _admin: AdminGuard) -> Result<Status, Error> {
    let user_id = user.user_id.clone();
    let user_id = Uuid::from_str(&user_id);
    match user_id {
        Ok(user_id) => {
            let query = r#"
                DELETE auths WHERE in == $user;
                DELETE $user;
            "#;
            DB.query(query)
                .bind(("user", Thing::from(("account", Id::Uuid(user_id)))))
                .await?;
            Ok(Status::Ok)
        },
        Err(_) => Err(Error::Custom(Status::BadRequest))
    }
}