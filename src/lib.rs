pub mod subscription;

pub use subscription::{Duration, Subscription};

pub fn get_total_cost(subscriptions: &Vec<Subscription>) {
    let mut total_cost: f32 = 0.0;
    for sub in subscriptions {
        println!(
            "Your subscription for {} at ${} per {:?} starting {}",
            sub.title,
            sub.dollar_price(),
            sub.duration,
            sub.start_date
        );
        let yearly_cost = sub.yearly_cost();
        println!(
            "Your {} subscription yearly cost is ${}",
            sub.title, yearly_cost
        );

        total_cost = total_cost + yearly_cost;
    }

    println!("Total yearly cost ${total_cost}");
}
