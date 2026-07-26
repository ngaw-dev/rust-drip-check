#![allow(unused_imports)]

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use diesel::prelude::*;
use rust_drip_check::db::{self, DbPool};
use rust_drip_check::models::NewSubscription;
use rust_drip_check::schema::subscriptions;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Clone)]
struct AppState {
    db: DbPool,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Visit api endpoints for data" }))
        .nest("/api/v1/subscriptions", subscription_routes())
        .with_state(AppState {
            db: db::establish_pool(),
        });

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
        .route(
            "/{id}/reminders/",
            get(show_reminders).post(create_reminder),
        )
        .route(
            "/{id}/reminders/{reminder_id}",
            get(get_reminder)
                .put(update_reminder)
                .delete(delete_reminder),
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

async fn create_subscription(
    State(state): State<AppState>,
    Json(body): Json<NewSubscription>,
) -> Json<Value> {
    let mut conn = state
        .db
        .get()
        .expect("ERROR: getting db connection from pool");

    let subscription_id = diesel::insert_into(subscriptions::table)
        .values(&body)
        .returning(subscriptions::id)
        .get_result::<i32>(&mut conn)
        .expect("ERROR: saving new subscription");

    println!("Saved subscription with id {}", subscription_id);

    Json(json!({
        "data": {
            "id": subscription_id
        }
    }))
}

async fn get_subscription(Path(id): Path<u16>) -> Json<Value> {
    Json(json!({
        "data": {
            "key": format!("TODO: get_subscription for {}", id)
        }
    }))
}

async fn update_subscription(Path(id): Path<u16>) -> Json<Value> {
    Json(json!({
        "data": {
            "key": format!("TODO: update_subscription {}", id)
        }
    }))
}

async fn delete_subscription(Path(id): Path<u16>) -> Json<Value> {
    Json(json!({
        "data": {
            "key": format!("TODO: delete_subscription {}", id)
        }
    }))
}

async fn show_reminders() -> Json<Value> {
    Json(json!({
        "data": {
            "key": "TODO: show_reminders"
        }
    }))
}

#[derive(Deserialize, Debug)]
struct ReminderParams {
    id: u32,
    reminder_id: u32,
}

async fn get_reminder(Path(params): Path<ReminderParams>) -> Json<Value> {
    Json(json!({
        "data": {
            "key": format!("TODO: get_reminder for {} reminder_id {}", params.id, params.reminder_id)
        }
    }))
}

async fn create_reminder(Path(params): Path<ReminderParams>) -> Json<Value> {
    Json(json!({
        "data": {
            "key": format!("TODO: create_reminder for {} reminder_id {}", params.id, params.reminder_id)
        }
    }))
}

async fn update_reminder(Path(params): Path<ReminderParams>) -> Json<Value> {
    Json(json!({
        "data": {
            "key": format!("TODO: update_reminder for {} reminder_id {}", params.id, params.reminder_id)
        }
    }))
}

async fn delete_reminder(Path(params): Path<ReminderParams>) -> Json<Value> {
    Json(json!({
        "data": {
            "key": format!("TODO: delete_reminder for {} reminder_id {}", params.id, params.reminder_id)
        }
    }))
}
