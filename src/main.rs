mod db;
mod types;
mod routes;
mod services;
mod cors;

use std::env;
use std::sync::LazyLock;
use rocket::{launch, routes};
use rocket::fs::{relative, FileServer};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::cors::CORS;
use crate::db::init::init;
use crate::routes::{auth_account, create_account, create_app, login_account, get_app, verify_account, authorize_app};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[launch]
pub async fn rocket() -> _ {
    unsafe{
        env::set_var("ROCKET_PORT", "8000");
        init().await.expect("Something went wrong!");
        rocket::build()
            .attach(CORS)
            .mount(
            "/",
            routes![
                cors::preflight_cors,
                create_account::create_account,
                login_account::login_account,
                verify_account::verify_account,
                auth_account::auth_account,
                create_app::create_app,
                authorize_app::authorize_app,
                get_app::get_app
            ],
        ).mount("/site", FileServer::from(relative!("www/auth-frontend/dist")))
    }
}
