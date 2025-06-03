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
struct ChangePasswordRequest {
    user_id: String,
    password: String,
}

/// Change user password route handler function for changing user password on demand.
#[post("/user/change_password", data = "<user>")]
pub async fn change_user_password(user: Json<ChangePasswordRequest>, _admin: AdminGuard) -> Result<Status, Error> {
    let user_id = user.user_id.clone();
    if &user.password.len() <= &3 {
        return Err(Error::Custom(Status::BadRequest));
    }
    match check_user_existence(&user_id).await{
        Ok(_) => {
            let user_id = Uuid::from_str(&user_id);
            let user_id = user_id.unwrap_or_else(|_| Uuid::new());
            let password = user.password.clone();
            let query = r#"
                UPDATE $user SET password = crypto::argon2::generate($password);
            "#;
            DB.query(query)
                .bind(("user", Thing::from(("account", Id::Uuid(user_id)))))
                .bind(("password", password))
                .await?;
            Ok(Status::Ok)
        },
        Err(_e) => Err(Error::Custom(Status::BadRequest))
    }
}