mod db;
mod types;
mod routes;
mod services;
mod cors;
mod guards;
mod tests;

use std::env;
use std::sync::LazyLock;
use rocket::{launch, routes, Build, Rocket};
use rocket::fs::{relative, FileServer};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::cors::CORS;
use crate::db::init::init;
use crate::routes::{auth_account, create_account, create_app, login_account, get_app, verify_account, authorize_app, panel, site_handler};
use crate::routes::panel::{change_app_id, change_app_perms, change_app_url, change_user_info, change_user_jwt, change_user_password, get_apps, get_users, rm_app, rm_user};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);


/// Set up rocket server, init database connection, bind routes and static files.
#[launch]
pub async fn rocket() -> Rocket<Build> {
    unsafe{
        env::set_var("ROCKET_PORT", "8000");
        init().await.expect("Something went wrong!");
        rocket::build()
            .attach(CORS)
            .mount("/", routes![
                cors::preflight_cors,
                create_account::create_account,
                login_account::login_account,
                verify_account::verify_account,
                auth_account::auth_account,
                create_app::create_app,
                authorize_app::authorize_app,
                get_app::get_app
            ], )
            .mount("/panel", routes![
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
            .mount("/site", FileServer::from(relative!("www/auth-frontend/dist")).rank(10))
            .mount("/site", routes![site_handler::site_handler])
    }
}
