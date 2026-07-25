use diesel::prelude::*;
use rust_drip_check::db::establish_connection;
use rust_drip_check::models::*;
use rust_drip_check::schema::subscriptions::dsl::*;

fn main() {
    let connection = &mut establish_connection();
    let results = subscriptions
        .limit(5)
        .select(Subscription::as_select())
        .load(connection)
        .expect("ERROR:  loading subscriptions");

    println!("Displaying {} subscriptions", results.len());
    for sub in results {
        println!(
            "(id: {:?}) Your subscription for {} at ${} per {:?} starting {}",
            sub.id,
            sub.title,
            sub.price as f32 / 100.0,
            sub.duration,
            sub.start_date
        );
        println!("-----------\n");
    }
}
