use rust_drip_check::{get_total_cost, Subscription};

fn main() {
    let file_path = "data.csv";
    let mut reader = csv::Reader::from_path(file_path).expect("ERROR: Could not open csv");

    let mut subscriptions = Vec::new();

    for data in reader.deserialize() {
        let record: Subscription = data.expect("ERROR: Could not read record");
        subscriptions.push(record);
    }

    get_total_cost(&subscriptions);
}
