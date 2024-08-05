use axum::{
    routing::{get, post},
    Router,
};
mod handlers;
use handlers::{accept_form, accept_json, handler, handler_404, query, res_json, show_form};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/query", get(query))
        .route("/form", get(show_form).post(accept_form))
        .route("/json", post(accept_json))
        .route("/resjson", post(res_json))
        .nest_service("/assets2", ServeDir::new("assets2/test.html"))
        .nest_service("/assets3", ServeDir::new("assets3"))
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404);
    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
