use rocket::{post, Response};
use rocket::http::{ContentType, Status};
use rocket::response::{content, status, Responder};
use rocket::serde::json::Json;
use uuid::Uuid;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use crate::DB;
use crate::db::error::error::Error;
use crate::types::account::{Account, AccountResponse};

#[post("/account/register", data = "<account>")]
pub async fn create_account(account: Json<Account>) -> Result<status::Custom<Json<AccountResponse>>, Error> {
    // Hash the password before storing it
    let argon2 = Argon2::default();
    let password = account.password.clone();

    // Generate a random salt and hash the password
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::Custom(Status::InternalServerError))?;

    // Create account object with hashed password
    let account_to_create = Account {
        password: hashed_password.to_string(),
        ..account.into_inner()
    };

    // Insert account into the database
    let account_id = Uuid::now_v7(); // Unique account ID
    let account_result: Option<Account> = DB
        .create(("account", account_id))
        .content(account_to_create)
        .await?;

    match account_result {
        Some(acc) => {
            // Account successfully created, send back response
            Ok(status::Custom(Status::Created,
                              Json(AccountResponse::from(acc))))
        },
        None => {
            // Return a custom error if account creation failed
            Err(Error::Db)
        }
    }
}
