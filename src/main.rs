use std::fs;

fn main() {
    let file_path = "data.csv";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("{contents}");
}
