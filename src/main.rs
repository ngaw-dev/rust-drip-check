fn main() {
    let title = "Github";
    let price = 999; // We are $9.99 and storing as all integer instead of float
    let duration = "monthly";
    let start_date = "14/04/2026";

    // We want the price to be printed as $ price / 100
    let dollar_price:f32 = price as f32 / 100.0;
    println!("Your subscription for {title} at ${dollar_price} per {duration} starting {start_date}");
}
