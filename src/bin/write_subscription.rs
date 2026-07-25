use std::io::stdin;

use chrono::NaiveDate;
use diesel::prelude::*;
use rust_drip_check::db::establish_connection;
use rust_drip_check::models::Duration;
use rust_drip_check::models::NewSubscription;
use rust_drip_check::schema::subscriptions;
use strum::IntoEnumIterator;

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut price = String::new();
    let mut duration = String::new();
    let mut start_date = String::new();

    println!("What is the subscription title?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end().to_string();

    println!("\nWhat is the price in cents?");
    stdin().read_line(&mut price).unwrap();
    let price: i32 = price.trim_end().parse().expect("Invalid number");

    println!("\nWhat is the duration?");
    for (i, d) in Duration::iter().enumerate() {
        println!("{}] {}", i + 1, d);
    }
    stdin().read_line(&mut duration).unwrap();
    let duration_idx: usize = duration
        .trim_end()
        .parse()
        .expect("ERROR: Invalid selection");
    let duration = Duration::from_index(duration_idx)
        .expect("ERROR: Invalid selection")
        .to_string();

    println!("\nWhat is the start date? (YYYY-MM-DD)");
    stdin().read_line(&mut start_date).unwrap();
    let parsed_date: NaiveDate = NaiveDate::parse_from_str(start_date.trim(), "%Y-%m-%d")
        .expect("ERROR: Invalid date format");

    let new_sub = NewSubscription {
        title,
        price,
        duration,
        start_date: parsed_date.format("%Y-%m-%d").to_string(),
    };

    diesel::insert_into(subscriptions::table)
        .values(&new_sub)
        .execute(connection)
        .expect("ERROR:  saving new subscription");

    println!("\nSaved subscription {}", new_sub.title);
}
