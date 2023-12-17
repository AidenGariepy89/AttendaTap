use askama::Template;
use axum::{
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use thiserror::Error;

/// Main Error Type for the program
#[derive(Error, Debug)]
pub enum Error {
    #[error("Static Error: {0:?}")]
    Static(String),

    #[error("Template Error: {0:?}")]
    TemplateError(String),
}

/// Allows Error to be converted into an axum::response::Response
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> Error IntoResponse {:?>12}", self);

        match self {
            Self::Static(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::TemplateError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

/// Result shorthand for the server
type Result<T> = core::result::Result<T, Error>;

/// into_response function for askama::Template
pub fn into_response<T: Template>(t: &T) -> Result<Response> {
    match t.render() {
        Ok(body) => {
            let headers = [(header::CONTENT_TYPE, HeaderValue::from_static(T::MIME_TYPE))];

            Ok((headers, body).into_response())
        }
        Err(err) => {
            Err(Error::TemplateError(err.to_string()))
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:42069")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Template for index.html
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: &'static str,
}

/// index route
async fn index() -> Result<Response> {
    let names = [
        "Aiden",
        "Kyle",
        "Andrew",
        "Ethan",
    ];

    let index = rand::random::<usize>() % 4;
    let name = names[index];

    into_response(&IndexTemplate { name })
}
