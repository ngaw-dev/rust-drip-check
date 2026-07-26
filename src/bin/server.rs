#![allow(unused_imports)]

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use diesel::prelude::*;
use rust_drip_check::db::{self, DbPool};
use rust_drip_check::models::{NewSubscription, Subscription};
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

async fn show_subscriptions(State(state): State<AppState>) -> Json<Value> {
    let mut conn = state
        .db
        .get()
        .expect("ERROR: getting db connection from pool");

    let results = subscriptions::table
        .select(Subscription::as_select())
        .load::<Subscription>(&mut conn)
        .expect("ERROR: loading subscriptions");

    Json(json!({ "data": results }))
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

async fn get_subscription(State(state): State<AppState>, Path(id): Path<i32>) -> Json<Value> {
    let mut conn = state
        .db
        .get()
        .expect("ERROR: getting db connection from pool");

    let subscription = subscriptions::table
        .find(id)
        .select(Subscription::as_select())
        .first::<Subscription>(&mut conn)
        .optional()
        .expect("ERROR: loading subscriptions");

    match subscription {
        Some(sub) => Json(json!({
            "data": sub
        })),
        None => Json(json!({ "data": null, "error": format!("Subscription {} not found", id) })),
    }
}

async fn update_subscription(State(state): State<AppState>, Path(id): Path<i32>) -> Json<Value> {
    let mut conn = state
        .db
        .get()
        .expect("ERROR: getting db connection from pool");

    Json(json!({
        "data": {
            "key": format!("TODO: update_subscription {}", id)
        }
    }))
}

async fn delete_subscription(State(state): State<AppState>, Path(id): Path<i32>) -> Json<Value> {
    let mut conn = state
        .db
        .get()
        .expect("ERROR: getting db connection from pool");

    let deleted = diesel::delete(subscriptions::table.find(id))
        .returning(Subscription::as_returning())
        .get_result::<Subscription>(&mut conn)
        .optional()
        .expect("ERROR: deleting subscription");

    match deleted {
        Some(sub) => {
            Json(json!({ "data": sub, "message": format!("Subscription {} deleted", id) }))
        }
        None => Json(json!({ "data": null, "error": format!("Subscription {} not found", id) })),
    }
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
