use rocket::{post, Response};
use rocket::http::{ContentType, Status};
use rocket::response::{content, status, Responder};
use rocket::serde::json::Json;
use uuid::Uuid;
use crate::DB;
use crate::db::error::error::Error;
use crate::types::login::{LoginQuery, LoginRequest, LoginResponse};

#[post("/account/login", data = "<account>")]
pub async fn login_account(account: Json<LoginRequest>) -> Result<status::Custom<Json<LoginResponse>>, Error> {
    let query = r#"
        SELECT id, jwt
        FROM account
        WHERE email = $email
          AND crypto::argon2::compare(password, $pass);
    "#;
    let email = account.email.clone();
    let password = account.password.clone();
    let mut q_res = DB
        .query(query)
        .bind(("email", email))
        .bind(("pass", password))
        .await?;
    println!("{:?}", q_res);
    let account_data: Option<LoginQuery> = q_res.take(0)?;
    match account_data {
        Some(account) => {
            let account: LoginResponse = account.into();
            Ok(status::Custom(Status::Ok, Json(account)))
        },
        None => Err(Error::Db)
    }
}