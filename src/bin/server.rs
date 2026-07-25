#[derive(Clone)]
struct AppState;

use axum::{
    Json, Router,
    routing::{delete, get, post, put},
};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Visit api endpoints for data" }))
        .nest("/api/v1/subscriptions", subscription_routes())
        .with_state(AppState);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn subscription_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(show_subscriptions).post(create_subscription))
        .route(
            "/{id}",
            get(get_subscription)
                .post(create_reminder)
                .put(update_subscription)
                .delete(delete_subscription),
        )
}

async fn show_subscriptions() -> Json<Value> {
    Json(json!({ 
        "data": {
            "id": 1,
            "title": "Subscriptions title",
            "price": 100,
            "duration": "Monthly",
            "start_date": "2026-04-14",
    } }))
}

async fn create_subscription() -> Json<Value> {
    Json(json!({
        "data": {
            "key": "TODO: create_subscription"
        }
    }))
}

async fn get_subscription() -> Json<Value> {
    Json(json!({
        "data": {
            "key": "TODO: get_subscription"
        }
    }))
}

async fn update_subscription() -> Json<Value> {
    Json(json!({
        "data": {
            "key": "TODO: update_subscription"
        }
    }))
}

async fn delete_subscription() -> Json<Value> {
    Json(json!({
        "data": {
            "key": "TODO: delete_subscription"
        }
    }))
}
