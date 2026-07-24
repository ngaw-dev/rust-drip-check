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

impl Subscription {
    fn yearly_cost(&self) -> f32 {
        let yearly_cost: f32;
        match self.duration {
            Duration::Weekly => yearly_cost = self.price as f32 * 52.0 / 100.0,
            Duration::Monthly => yearly_cost = self.price as f32 * 12.0 / 100.0,
            _ => {
                // duration is default let's assume yearly
                yearly_cost = self.price as f32 / 100.0
            }
        }

        return yearly_cost;
    }

    fn dollar_price(&self) -> f32 {
        return self.price as f32 / 100.0;
    }

    fn display(&self) {
        println!(
            "Your subscription for {} at ${} per {:?} starting {}",
            &self.title,
            &self.dollar_price(),
            &self.duration,
            &self.start_date
        );
    }
}

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

fn get_total_cost(subscriptions: &Vec<Subscription>) {
    let mut total_cost: f32 = 0.0;
    for sub in subscriptions {
        // We want the price to be printed as $ price / 100
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculate_yearly_cost() {
        let mut sub = Subscription {
            title: "Test title".to_string(),
            price: 1000,
            duration: Duration::Monthly,
            start_date: "08/10/2025".to_string(),
        };
        assert_eq!(sub.yearly_cost(), 120.0);

        sub.duration = Duration::Weekly;
        assert_eq!(sub.yearly_cost(), 520.0);
    }
}
