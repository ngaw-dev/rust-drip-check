use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::subscriptions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Subscription {
    pub id: i32,
    pub title: String,
    pub price: i32,
    pub duration: String,
    pub start_date: String,
}

use crate::schema::subscriptions;

#[derive(Insertable)]
#[diesel(table_name = subscriptions)]
pub struct NewSubscription {
    pub title: String,
    pub price: i32,
    pub duration: String,
    pub start_date: String,
}
