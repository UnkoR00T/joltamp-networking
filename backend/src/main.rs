mod db;
mod types;
mod routes;
mod services;
mod cors;
mod guards;

use std::env;
use std::sync::LazyLock;
use rocket::{launch, routes, Build, Rocket};
use rocket::fs::{relative, FileServer};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::cors::CORS;
use crate::db::init::init;
use crate::routes::{auth_account, create_account, create_app, login_account, get_app, verify_account, authorize_app, panel};
use crate::routes::panel::{change_app_id, change_app_perms, change_app_url, change_user_info, change_user_jwt, change_user_password, get_apps, get_users, rm_app, rm_user};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);


/// Set up rocket server, init database connection, bind routes and static files.
#[launch]
pub async fn rocket() -> Rocket<Build> {
    if (!cfg!(debug_assertions)) {
        tracing_subscriber::fmt()
            .with_env_filter("info")
            .with_writer(std::fs::File::create("/var/log/sso/sso-backend.log").unwrap())
            .init();
    }
    init().await.expect("Something went wrong!");
    rocket::build()
        .attach(CORS)
        .mount("/api/", routes![
            cors::preflight_cors,
            create_account::create_account,
            login_account::login_account,
            verify_account::verify_account,
            auth_account::auth_account,
            create_app::create_app,
            authorize_app::authorize_app,
            get_app::get_app
        ], )
        .mount("/api/panel", routes![
            get_users::get_users,
            get_apps::get_apps,
            change_app_perms::change_app_perms,
            change_app_id::change_app_id,
            change_user_jwt::change_user_jwt,
            rm_app::rm_app,
            rm_user::rm_user,
            change_user_info::change_user_info,
            change_user_password::change_user_password,
            change_app_url::change_app_url,
        ])
}
