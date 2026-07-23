fn main() {
    let title = "Github";
    let price = 999; // We are $9.99 and storing as all integer instead of float
    let duration = "monthly";
    let start_date = "14/04/2026";

    // We want the price to be printed as $ price / 100
    let dollar_price:f32 = price as f32 / 100.0;
    println!("Your subscription for {title} at ${dollar_price} per {duration} starting {start_date}");

    println!("Your {title} subscription yearly cost is ${}", calculate_yearly_cost(price, duration));

    let title2 = "OpenCode";
    let price2 = 250;
    let duration2 = "weekly";
    let start_date2 = "22/03/2026";
    
    let dollar_price2:f32 = price2 as f32 / 100.0;

    println!("Your subscription for {title2} at ${dollar_price2} per {duration2} starting {start_date2}");
    println!("Your {title2} subscription yearly cost is ${}", calculate_yearly_cost(price2, duration2));

    let total_cost = calculate_yearly_cost(price, duration) + calculate_yearly_cost(price2, duration2);
    println!("Your yearly drip cost ${total_cost}");
}

fn calculate_yearly_cost(price: i32, duration: &str) -> f32 {
    let yearly_cost:f32;
    if duration == "weekly" {
        yearly_cost = price as f32 * 52.0 / 100.0;
    } else if duration == "monthly" {
        yearly_cost = price as f32 * 12.0 / 100.0;
    } else { // duration is default let's assume yearly
        yearly_cost = price as f32 as f32 / 100.0;
    }

    return yearly_cost;
}
