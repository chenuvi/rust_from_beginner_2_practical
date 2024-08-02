use axum::response::Html;
use axum::routing::get;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    let app = Router::new()
        .route("/foo", get(handler))
        .nest_service("/assets2", ServeDir::new("assets2/test.html"))
        .nest_service("/assets3", ServeDir::new("assets3"))
        .fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
