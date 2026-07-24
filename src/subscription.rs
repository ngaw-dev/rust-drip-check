#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Duration {
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Subscription {
    pub title: String,
    pub price: i32,
    pub duration: Duration,
    pub start_date: String,
}

impl Subscription {
    pub fn yearly_cost(&self) -> f32 {
        let yearly_cost: f32;
        match self.duration {
            Duration::Weekly => yearly_cost = self.price as f32 * 52.0 / 100.0,
            Duration::Monthly => yearly_cost = self.price as f32 * 12.0 / 100.0,
            Duration::Quarterly => yearly_cost = self.price as f32 * 4.0 / 100.0,
            _ => yearly_cost = self.price as f32 / 100.0,
        }

        return yearly_cost;
    }

    pub fn dollar_price(&self) -> f32 {
        return self.price as f32 / 100.0;
    }

    pub fn display(&self) {
        println!(
            "Your subscription for {} at ${} per {:?} starting {}",
            &self.title,
            &self.dollar_price(),
            &self.duration,
            &self.start_date
        );
    }
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

        sub.duration = Duration::Quarterly;
        assert_eq!(sub.yearly_cost(), 40.0);
    }
}
