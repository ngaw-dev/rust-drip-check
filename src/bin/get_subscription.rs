use diesel::prelude::*;
use rust_drip_check::db::establish_connection;
use rust_drip_check::models::Subscription;
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
        Ok(Some(subscription)) => println!(
            "(id: {:?}) Your subscription for {} at ${} per {:?} starting {}",
            subscription.id,
            subscription.title,
            subscription.price as f32 / 100.0,
            subscription.duration,
            subscription.start_date
        ),
        Ok(None) => println!("WARNING: Unable to find subscription {}", subscription_id),
        Err(_) => println!(
            "ERROR: An error occured while fetching subscription {}",
            subscription_id
        ),
    }
}
