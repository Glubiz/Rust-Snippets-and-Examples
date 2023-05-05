use std::fs::File;

fn main() {
    match File::open("hello.txt") {
        Ok(file) => println!("File opened: {:?}", file),
        Err(error) => println!("Failed to open file: {:?}", error),
    }
}
