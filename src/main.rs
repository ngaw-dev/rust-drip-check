fn main() {
    let file_path = "data.csv";

    let mut reader = csv::Reader::from_path(file_path)
        .expect("ERROR: Could not open csv");

    for data in reader.records() {
        let record = data.expect("could not read record");
        println!("{:?}", record);
    }
}
