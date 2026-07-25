use chrono::NaiveDate;
use diesel::prelude::*;
use rust_drip_check::db::establish_connection;
use rust_drip_check::models::Duration;
use rust_drip_check::models::Subscription;
use std::env::args;
use std::io::stdin;
use strum::IntoEnumIterator;

fn main() {
    use rust_drip_check::schema::subscriptions::dsl::*;

    let subscription_id = args()
        .nth(1)
        .expect("INFO: Usage cargo run --bin update_subscription id")
        .parse::<i32>()
        .expect("ERROR: Invalid ID");

    let connection = &mut establish_connection();

    let subscription = subscriptions
        .find(subscription_id)
        .select(Subscription::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Subscription>, otherwise it will throw an error

    if let Ok(maybe_sub) = subscription {
        if let Some(subscription) = maybe_sub {
            // Subscription found
            let mut question = String::new();
            println!(
                "
                Which field to change 
                1] Title
                2] Price
                3] Duration
                4] Start date
            "
            );
            stdin().read_line(&mut question).unwrap();
            let option: i32 = question.trim_end().parse().expect("Invalid selection");
            if option < 1 || option > 4 {
                println!("ERROR: Invalid selection");
                return;
            }

            let mut val = String::new();
            println!("Enter value to change");
            stdin().read_line(&mut val).unwrap();
            let val = val.trim();
            let subscription: Subscription = match option {
                1 => diesel::update(subscriptions.find(subscription_id))
                    .set(title.eq(val))
                    .returning(Subscription::as_returning())
                    .get_result(connection)
                    .unwrap(),
                2 => {
                    let int_val = val.trim().parse::<i32>().expect("ERROR: Invalid price");
                    diesel::update(subscriptions.find(subscription_id))
                        .set(price.eq(int_val))
                        .returning(Subscription::as_returning())
                        .get_result(connection)
                        .unwrap()
                }
                3 => {
                    let mut dur = String::new();
                    println!("Select duration:");
                    for (i, d) in Duration::iter().enumerate() {
                        println!("{}] {}", i + 1, d);
                    }
                    stdin().read_line(&mut dur).unwrap();
                    let idx: usize = dur.trim_end().parse().expect("ERROR: Invalid selection");
                    let new_duration = Duration::from_index(idx).expect("ERROR: Invalid selection");
                    diesel::update(subscriptions.find(subscription_id))
                        .set(duration.eq(new_duration.to_string()))
                        .returning(Subscription::as_returning())
                        .get_result(connection)
                        .unwrap()
                }
                4 => {
                    let parsed_date = NaiveDate::parse_from_str(val.trim(), "%Y-%m-%d")
                        .expect("ERROR: Invalid date format");
                    diesel::update(subscriptions.find(subscription_id))
                        .set(start_date.eq(parsed_date.format("%Y-%m-%d").to_string()))
                        .returning(Subscription::as_returning())
                        .get_result(connection)
                        .unwrap()
                }
                _ => unreachable!(),
            };

            println!("Subscription updated {:?}", subscription);
        } else {
            println!("WARNING: Unable to find subscription {}", subscription_id);
        }
    } else {
        println!(
            "ERROR: An error occured while fetching subscription {}",
            subscription_id
        );
    }
}
