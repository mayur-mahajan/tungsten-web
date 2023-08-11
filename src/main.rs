use std::net::SocketAddr;
use axum::{Router, routing::get, Server};

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index));

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("Server started on {addr}");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "Hello rust!\r\n"
}