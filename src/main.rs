use rust_drip_check::{get_total_cost, Duration, Subscription};

fn main() {
    let mut subscriptions = Vec::new();

    subscriptions.push(Subscription {
        title: "Github".to_string(),
        price: 999,
        duration: Duration::Monthly,
        start_date: "14/04/2026".to_string(),
    });
    subscriptions.push(Subscription {
        title: "OpenCode".to_string(),
        price: 250,
        duration: Duration::Weekly,
        start_date: "22/03/2026".to_string(),
    });
    subscriptions.push(Subscription {
        title: "Amazon Prime".to_string(),
        price: 3000,
        duration: Duration::Yearly,
        start_date: "08/10/2025".to_string(),
    });

    get_total_cost(&subscriptions);

    println!("-- Removing the subscriptions at index 1 (start from 0) --");
    subscriptions.remove(1);
    get_total_cost(&subscriptions);

    println!("-- Adding new subscriptions --");
    subscriptions.push(Subscription {
        title: "Netflix".to_string(),
        price: 1549,
        duration: Duration::Monthly,
        start_date: "23/08/2025".to_string(),
    });
    get_total_cost(&subscriptions);

    println!("-- Show subscriptions where yearly price is > $5.00 --");
    let large_subscriptions = subscriptions.iter().filter(|sub| sub.price > 500);
    for sub in large_subscriptions {
        sub.display();
    }

    println!("-- Show subscriptions Monthly --");
    let monthly = subscriptions
        .iter()
        .filter(|sub| matches!(sub.duration, Duration::Monthly));
    for sub in monthly {
        sub.display();
    }

    println!("-- Show subscriptions with title like git --");
    let filtered = subscriptions
        .iter()
        .filter(|sub| sub.title.to_lowercase().contains("git"));
    for sub in filtered {
        sub.display();
    }
}
