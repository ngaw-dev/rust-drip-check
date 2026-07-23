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
    let subscriptions = [
        Subscription {
            title: "Github".to_string(),
            price: 999,
            duration: Duration::Monthly,
            start_date: "14/04/2026".to_string(),
        },
        Subscription {
            title: "OpenCode".to_string(),
            price: 250,
            duration: Duration::Weekly,
            start_date: "22/03/2026".to_string(),
        },
        Subscription {
            title: "Amazon Prime".to_string(),
            price: 3000,
            duration: Duration::Yearly,
            start_date: "08/10/2025".to_string(),
        },
    ];

    let mut total_cost: f32 = 0.0;
    for sub in subscriptions {
        // We want the price to be printed as $ price / 100
        let dollar_price: f32 = sub.price as f32 / 100.0;
        println!(
            "Your subscription for {} at ${dollar_price} per {:?} starting {}",
            sub.title, sub.duration, sub.start_date
        );
        let yearly_cost = calculate_yearly_cost(sub.price, sub.duration);
        println!(
            "Your {} subscription yearly cost is ${}",
            sub.title, yearly_cost
        );

        total_cost = total_cost + yearly_cost;
    }

    println!("Total yearly cost ${total_cost}")
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
