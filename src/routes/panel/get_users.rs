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

#[get("/users")]
pub async fn get_users(admin: AdminGuard) -> Result<status::Custom<Json<Vec<AccountSafe>>>, Error> {
    if(admin.0) {
        let query = r#"
                    SELECT * FROM account;
                "#;
        let mut q_res = DB
            .query(query)
            .await?;
        let accounts: Vec<AccountSafe> = q_res.take(0)?;
        Ok(status::Custom(Status::Ok, Json(accounts)))
    }else{
        Err(Error::Custom(Status::Unauthorized))
    }
}