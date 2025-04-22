use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::DB;
use crate::db::error::error::Error;
use crate::routes::auth_account::auth_account;
use crate::types::account::AccountSafe;
use crate::types::app::CreateAppRequest;
use crate::guards::auth_guard::AuthToken;
use crate::guards::networking_admin_guard::AdminGuard;

#[get("/users?<page>&<limit>")]
pub async fn get_users(_admin: AdminGuard, page: i32, limit: i32) -> Result<status::Custom<Json<Vec<AccountSafe>>>, Error> {
    if limit > 100 {
        return Err(Error::Custom(Status::UnprocessableEntity));
    }
    let query = r#"
                SELECT * FROM account START $start LIMIT $limit;
            "#;
    let mut q_res = DB
        .query(query)
        .bind(("start", (page - 1) * limit))
        .bind(("limit", limit))
        .await?;
    let accounts: Vec<AccountSafe> = q_res.take(0)?;
    Ok(status::Custom(Status::Ok, Json(accounts)))
}