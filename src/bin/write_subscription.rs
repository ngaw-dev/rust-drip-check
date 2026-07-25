use std::io::stdin;

use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;
use rust_drip_check::db::establish_connection;
use rust_drip_check::models::{Duration, NewReminder, NewSubscription};
use rust_drip_check::schema::{reminders, subscriptions};
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

    let subscription_id = diesel::insert_into(subscriptions::table)
        .values(&new_sub)
        .returning(subscriptions::id)
        .get_result::<i32>(connection)
        .expect("ERROR: saving new subscription");

    println!("\nSaved subscription with id {}", subscription_id);

    // Add reminders
    loop {
        let mut days_before_input = String::new();
        println!("\nDays before expiration to remind? (enter 0 to exit)");
        stdin().read_line(&mut days_before_input).unwrap();
        let days_before: i32 = days_before_input
            .trim_end()
            .parse()
            .expect("Invalid number");

        if days_before == 0 {
            break;
        }

        let mut reminder_time_input = String::new();
        println!("Reminder time? (HH:MM format, e.g., 09:00)");
        stdin().read_line(&mut reminder_time_input).unwrap();
        let reminder_time = reminder_time_input.trim_end().to_string();

        // Validate time format
        NaiveTime::parse_from_str(&reminder_time, "%H:%M")
            .expect("ERROR: Invalid time format. Use HH:MM");

        let new_reminder = NewReminder {
            subscription_id,
            days_before,
            reminder_time: reminder_time.clone(),
        };

        diesel::insert_into(reminders::table)
            .values(&new_reminder)
            .execute(connection)
            .expect("ERROR: saving new reminder");

        println!(
            "Saved reminder: {} days before at {}",
            days_before, reminder_time
        );
    }

    println!("\nDone!");
}
