pub mod error {
    use rocket::http::Status;
    use rocket::response::{self, Responder, Response};
    use rocket::Request;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error{
        #[error("Database error")]
        Db,
        #[error("Request failed")]
        Custom(Status)
    }

    impl<'r> Responder<'r, 'static> for Error{
        fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
            match self {
                Error::Db => {
                    let error_message = format!(r#"{{ "error": "{self}" }}"#);
                    Response::build()
                        .status(Status::InternalServerError)
                        .header(rocket::http::ContentType::JSON)
                        .sized_body(error_message.len(), std::io::Cursor::new(error_message))
                        .ok()
                }
                Error::Custom(status) => {
                    let error_message = format!(r#"{{ "error": "{self}" }}"#);
                    Response::build()
                        .status(status)
                        .header(rocket::http::ContentType::JSON)
                        .sized_body(error_message.len(), std::io::Cursor::new(error_message))
                        .ok()
                }
            }
        }
    }
    impl From<surrealdb::Error> for Error {
        fn from(error: surrealdb::Error) -> Self {
            eprintln!("{error}");
            Self::Db
        }
    }
}