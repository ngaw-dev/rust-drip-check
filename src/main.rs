use csv::Writer;
use rust_drip_check::{Duration, Subscription, get_total_cost};

fn main() {
    let file_path = "data.csv";
    let mut reader = csv::Reader::from_path(file_path).expect("ERROR: Could not open csv");

    let mut subscriptions = Vec::new();

    for data in reader.deserialize() {
        let record: Subscription = data.expect("ERROR: Could not read record");
        subscriptions.push(record);
    }

    get_total_cost(&subscriptions);

    println!("-- Removing the subscriptions at index 1 (start from 0) --");
    subscriptions.remove(1);
    get_total_cost(&subscriptions);

    println!("-- Over-writing the CSV file with the removed data --");
    let mut wtr = Writer::from_path("data.csv").expect("ERROR: File is not writable");
    // let headers = reader.headers().expect("ERROR: Headers not found").clone();
    for record in &subscriptions {
        wtr.serialize(record)
            .expect("ERROR: Could not write record");
    }
    wtr.flush().expect("ERROR: Could not flush records");

    subscriptions.push(Subscription {
        title: "New subscription".to_string(),
        price: 1000,
        duration: Duration::Monthly,
        start_date: "25/03/2026".to_string(),
    });
    println!("-- Over-writing the CSV file with the new added data --");
    let mut wtr = Writer::from_path("data.csv").expect("ERROR: File is not writable");
    // let headers = reader.headers().expect("ERROR: Headers not found").clone();
    for record in &subscriptions {
        wtr.serialize(record)
            .expect("ERROR: Could not write record");
    }
    wtr.flush().expect("ERROR: Could not flush records");
}
