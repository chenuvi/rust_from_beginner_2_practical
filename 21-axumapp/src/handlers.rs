use askama::Template;
use axum::extract::Path;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
    Form, Json,
};
use serde::Deserialize;
use serde_json::json;
pub async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Params {
    foo: i32,
    bar: String,
    aa: Option<i32>,
}

pub async fn query(Query(params): Query<Params>) -> impl IntoResponse {
    tracing::debug!("query params {:?}", params);
    Html("<h3>Test query</h3>")
}

pub async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/form" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>

                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
    "#,
    )
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Input {
    name: String,
    email: String,
}

pub async fn accept_form(Form(input): Form<Input>) -> Html<&'static str> {
    tracing::debug!("form input {:?}", input);
    Html("<h3>Form posted</h3>")
}
pub async fn accept_json(Json(input): Json<Input>) -> Html<&'static str> {
    tracing::debug!("json params {:?}", input);
    Html("<h3>Json posted x1</h3>")
}
pub async fn res_json(Json(input): Json<Input>) -> impl IntoResponse {
    tracing::debug!("json params {:?}", input);
    Json(json!({
        "result": "ok",
        "number": 1,
    }))
}

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
    name: String,
}
pub async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    HelloTemplate { name }.to_string()
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here!")
}
