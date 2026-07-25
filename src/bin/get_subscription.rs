use diesel::prelude::*;
use rust_drip_check::db::establish_connection;
use rust_drip_check::models::{Reminder, Subscription};
use std::env::args;

fn main() {
    use rust_drip_check::schema::subscriptions::dsl::*;

    let subscription_id = args()
        .nth(1)
        .expect("INFO: Usage cargo run --bin get_subscription id")
        .parse::<i32>()
        .expect("ERROR: Invalid ID");

    let connection = &mut establish_connection();

    let subscription = subscriptions
        .find(subscription_id)
        .select(Subscription::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Subscription>, otherwise it will throw an error

    match subscription {
        Ok(Some(subscription)) => {
            let subscription_reminders = Reminder::belonging_to(&subscription)
                .select(Reminder::as_select())
                .load(connection)
                .expect("ERROR: loading reminders");

            println!(
                "(id: {:?}) Your subscription for {} at ${} per {:?} starting {}",
                subscription.id,
                subscription.title,
                subscription.price as f32 / 100.0,
                subscription.duration,
                subscription.start_date,
            );
            println!("Reminders: ");
            for reminder in subscription_reminders {
                println!(
                    "{} days before at {} ",
                    reminder.days_before, reminder.reminder_time
                );
            }
        }
        Ok(None) => println!("WARNING: Unable to find subscription {}", subscription_id),
        Err(_) => println!(
            "ERROR: An error occured while fetching subscription {}",
            subscription_id
        ),
    }
}
