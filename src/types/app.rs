use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateAppRequest{
    pub(crate) name: String,
    perms: Vec<String>,
    url: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct App{
    id: Thing,
    name: String,
    perms: Vec<String>,
    #[serde(flatten)]
    url: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct CreateAppResponse {
    id: Id,
}

impl From<App> for CreateAppResponse{
    fn from(value: App) -> Self {
        CreateAppResponse{
            id: value.id.id
        }
    }
}