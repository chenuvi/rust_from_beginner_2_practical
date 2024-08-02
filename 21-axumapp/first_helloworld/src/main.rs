use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1> <img src='https://learn.lianglianglee.com/%E4%B8%93%E6%A0%8F/%E9%99%88%E5%A4%A9%20%C2%B7%20Rust%20%E7%BC%96%E7%A8%8B%E7%AC%AC%E4%B8%80%E8%AF%BE/assets/1cf03ee698cyy875e8fac45b8ed5f88d.jpg'> <p>rust is interesting</p>")
}
