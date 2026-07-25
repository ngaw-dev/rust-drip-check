use axum::{
    Json, Router,
    routing::{delete, get, post, put},
};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(get_subscription));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_subscription() -> Json<Value> {
    Json(json!({ 
        "data": {
            "id": 1,
            "title": "Subscriptions title",
            "price": 100,
            "duration": "Monthly",
            "start_date": "2026-04-14",
    } }))
}
