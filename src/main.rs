use axum::{
    body::Body,
    routing::get,
    response::Json,
    Router,
};
use serde_json::{json, Value};
use tower::ServiceBuilder;

use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    // our router
    let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar))
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            
    )
    ;
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

// which calls one of these handlers
async fn root() -> &'static str {
    "Hello world"
}
async fn get_foo() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
async fn post_foo() {}
async fn foo_bar() {}