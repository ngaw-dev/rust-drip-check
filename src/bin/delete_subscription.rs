use diesel::prelude::*;
use rust_drip_check::db::establish_connection;
use std::env::args;

fn main() {
    use rust_drip_check::schema::subscriptions::dsl::*;

    let subscription_id = args()
        .nth(1)
        .expect("INFO: Usage cargo run --bin delete_subscription id")
        .parse::<i32>()
        .expect("ERROR: Invalid ID");

    let connection = &mut establish_connection();

    let delete_count = diesel::delete(subscriptions.filter(id.eq(subscription_id)))
        .execute(connection)
        .expect("ERROR: Deleting record");

    if delete_count > 0 {
        println!(
            "Deleted id {} subscription. Total deleted {:?}",
            subscription_id, delete_count
        );
    } else {
        println!("WARNING: Record not found");
    }
}
