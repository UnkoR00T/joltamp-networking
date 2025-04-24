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
struct ChangeInfoRequest {
    user_id: String,
    firstname: Option<String>,
    lastname: Option<String>,
    email: Option<String>,
}

/// Change user info route handler function for changing user info such: firstname, lastname, email.
#[post("/user/change_info", data = "<user>")]
pub async fn change_user_info(user: Json<ChangeInfoRequest>, _admin: AdminGuard) -> Result<Status, Error> {
    let user_id = Uuid::from_str(&user.user_id);
    let user_id = user_id.unwrap_or_else(|_| Uuid::new());
    match check_user_existence(&user.user_id.to_string()).await {
        Ok(_) => {
            let mut query = String::from("UPDATE $user SET");
            let mut updates = Vec::new();

            if let Some(firstname) = &user.firstname {
                updates.push(format!("firstname = '{}'", firstname));
            }
            if let Some(lastname) = &user.lastname {
                updates.push(format!("lastname = '{}'", lastname));
            }
            if let Some(email) = &user.email {
                if email.contains("@"){
                    updates.push(format!("email = '{}'", email));
                }else{
                    return Err(Error::Custom(Status::UnprocessableEntity));
                }
            }

            // Join all updates with commas
            if !updates.is_empty() {
                query.push(' ');
                query.push_str(&updates.join(", "));
            }

            query.push(';');

            DB.query(query)
                .bind(("user", Thing::from(("account", Id::Uuid(user_id)))))
                .await?;

            Ok(Status::Ok)
        }
        Err(_) => Err(Error::Custom(Status::BadRequest)),
    }
}