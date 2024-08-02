use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    foo: i32,
    bar: String,
    aa: Option<i32>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/query", get(query))
        .nest_service("/assets2", ServeDir::new("assets2/test.html"))
        .nest_service("/assets3", ServeDir::new("assets3"))
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http());
    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn query(Query(params): Query<Params>) -> impl IntoResponse {
    tracing::debug!("query params {:?}", params);
    Html("<h3>Test query</h3>")
}
