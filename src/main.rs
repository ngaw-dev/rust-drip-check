use rust_drip_check::Subscription;

fn main() {
    let file_path = "data.csv";

    let mut reader = csv::Reader::from_path(file_path).expect("ERROR: Could not open csv");

    for data in reader.deserialize() {
        let record: Subscription = data.expect("ERROR: Could not read record");
        println!("{:?}", record);
    }
}
