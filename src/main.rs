#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", axum::routing::get(handler_top))
        .route("/other", axum::routing::get(handler_other));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_top() -> String {
    "Hello, World!".to_string()
}

async fn handler_other() -> String {
    "This is other page...".to_string()
}
