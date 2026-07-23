struct Subscription {
    title: String,
    price: i32,
    duration: Duration,
    start_date: String,
}

#[derive(Debug)]
enum Duration {
    Weekly,
    Monthly,
    Yearly,
}

fn main() {
    let sub1 = Subscription {
        title: "Github".to_string(),
        price: 999,
        duration: Duration::Monthly,
        start_date: "14/04/2026".to_string(),
    };
    let sub2 = Subscription {
        title: "OpenCode".to_string(),
        price: 250,
        duration: Duration::Weekly,
        start_date: "22/03/2026".to_string(),
    };

    // We want the price to be printed as $ price / 100
    let dollar_price: f32 = sub1.price as f32 / 100.0;
    println!(
        "Your subscription for {} at ${dollar_price} per {:?} starting {}",
        sub1.title, sub1.duration, sub1.start_date
    );
    println!(
        "Your {} subscription yearly cost is ${}",
        sub1.title,
        calculate_yearly_cost(sub1.price, sub1.duration)
    );

    let dollar_price2: f32 = sub2.price as f32 / 100.0;
    println!(
        "Your subscription for {} at ${dollar_price2} per {:?} starting {}",
        sub2.title, sub2.duration, sub2.start_date
    );
    println!(
        "Your {} subscription yearly cost is ${}",
        sub2.title,
        calculate_yearly_cost(sub2.price, sub2.duration)
    );
}

fn calculate_yearly_cost(price: i32, duration: Duration) -> f32 {
    let yearly_cost: f32;
    match duration {
        Duration::Weekly => yearly_cost = price as f32 * 52.0 / 100.0,
        Duration::Monthly => yearly_cost = price as f32 * 12.0 / 100.0,
        _ => {
            // duration is default let's assume yearly
            yearly_cost = price as f32 as f32 / 100.0
        }
    }

    return yearly_cost;
}
