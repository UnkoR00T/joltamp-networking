#[cfg(test)]
mod tests {
    use rocket::{local::asynchronous::Client, http::{Status, ContentType}};
    use serde_json::json;
    use crate::rocket;

    #[rocket::async_test]
    async fn test_not_found() {
        let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
        let res = client.get("/totalna/glupota/dla/notfound").dispatch().await;
        assert_eq!(res.status(), Status::NotFound);
    }

    #[rocket::async_test]
    async fn test_register() {
        let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
        let payload = json!({
            "firstname": "test",
            "lastname": "test",
            "email": "test", // Should fail because of incorrect email
            "password": "test"
        });
        let res = client.post("/account/register")
            .header(ContentType::JSON)
            .body(payload.to_string())
            .dispatch()
            .await;
        assert_eq!(res.status(), Status::InternalServerError);
        let payload = json!({
            "firstname": "test",
            "email": "test", // Should fail because of missing fields
        });
        let res = client.post("/account/register")
            .header(ContentType::JSON)
            .body(payload.to_string())
            .dispatch()
            .await;
        assert_eq!(res.status(), Status::UnprocessableEntity);
    }

    #[rocket::async_test]
    async fn test_login() {
        let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
        let payload = json!({
            "email": "test@gmail.com",
            "password": "test"
        });
        let res = client.post("/account/login")
            .header(ContentType::JSON)
            .body(payload.to_string())
            .dispatch()
            .await;
        assert_eq!(res.status(), Status::InternalServerError);
    }
}
