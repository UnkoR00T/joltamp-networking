use argon2::PasswordVerifier;
use rocket::{post, Response};
use rocket::http::{ContentType, Status};
use rocket::response::{content, status, Responder};
use rocket::serde::json::Json;
use surrealdb::sql::Thing;
use uuid::Uuid;
use crate::DB;
use crate::db::error::error::Error;
use crate::types::login::{LoginQuery, LoginRequest, LoginResponse};

#[post("/account/login", data = "<account>")]
pub async fn login_account(account: Json<LoginRequest>) -> Result<status::Custom<Json<LoginResponse>>, Error> {
    // Define the SQL query
    let query = r#"
        SELECT id, password, jwt
        FROM account
        WHERE email = $email;
    "#;

    let email = account.email.clone();
    let password = account.password.clone(); // The plain text password provided by the user

    // Query the database for the account with the provided email
    let mut q_res = DB
        .query(query)
        .bind(("email", email))
        .await?;

    // Fetch the first matching account
    let account_data: Option<LoginQuery> = q_res.take(0)?;

    match account_data {
        Some(account) => {
            // Use Argon2 to verify the password
            let argon2 = argon2::Argon2::default();
            let parsed_hash = argon2::password_hash::PasswordHash::new(&account.password)
                .map_err(|_| Error::Custom(Status::InternalServerError))?;

            // Check if the password matches the stored hash
            let verify_result = argon2.verify_password(password.as_bytes(), &parsed_hash);

            if verify_result.is_ok() {
                // Password is correct, return the JWT token
                Ok(status::Custom(Status::Ok, Json(LoginResponse::from(account))))
            } else {
                // Password does not match
                Err(Error::Custom(Status::BadRequest))
            }
        },
        None => Err(Error::Custom(Status::BadRequest)),  // No account found with the provided email
    }
}
