use rocket::yansi::Paint;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use crate::DB;

pub async fn init() -> Result<(), surrealdb::Error> {
    DB.connect::<Ws>("localhost:9952").await?;
    DB.signin(Root{
        username: "root",
        password: "root"
    }).await?;
    DB.use_ns("Network").use_db("Users").await?;
    DB.query("
            DEFINE TABLE IF NOT EXISTS account;
            DEFINE FIELD IF NOT EXISTS id ON TABLE account TYPE string;
            DEFINE FIELD IF NOT EXISTS firstname ON TABLE account TYPE string;
            DEFINE FIELD IF NOT EXISTS lastname ON TABLE account TYPE string;
            DEFINE FIELD IF NOT EXISTS email ON TABLE account TYPE string ASSERT string::is::email($value);
            DEFINE INDEX IF NOT EXISTS email_index ON TABLE account FIELDS email UNIQUE;
            DEFINE FIELD IF NOT EXISTS password ON TABLE account TYPE string VALUE crypto::argon2::generate($value);
            DEFINE FIELD IF NOT EXISTS jwt ON TABLE account TYPE uuid DEFAULT ALWAYS rand::uuid::v7() VALUE rand::uuid::v7();

            DEFINE TABLE IF NOT EXISTS auth_apps;
            DEFINE FIELD IF NOT EXISTS name ON TABLE auth_apps TYPE string;
            DEFINE FIELD IF NOT EXISTS perms ON TABLE auth_apps TYPE array<string>;

            DEFINE TABLE IF NOT EXISTS auths TYPE RELATION IN account OUT auth_apps SCHEMALESS PERMISSIONS NONE;
            DEFINE FIELD IF NOT EXISTS in ON auths TYPE record<account> PERMISSIONS FULL;
            DEFINE FIELD IF NOT EXISTS out ON auths TYPE record<auth_apps> PERMISSIONS FULL;

             ", ).await?;
    Ok(())
}