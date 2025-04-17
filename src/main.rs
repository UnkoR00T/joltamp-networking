mod db;
mod types;
mod routes;

use std::env;
use std::sync::LazyLock;
use rocket::{launch, routes};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::db::init::init;
use crate::routes::{auth_account, create_account, login_account};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[launch]
pub async fn rocket() -> _ {
    unsafe{
        env::set_var("ROCKET_PORT", "8080");
        init().await.expect("Something went wrong!");
        rocket::build().mount(
            "/",
            routes![
                create_account::create_account,
                login_account::login_account,
                auth_account::auth_account,
            ],
        )
    }
}
