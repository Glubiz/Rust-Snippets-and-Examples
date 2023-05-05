use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file_path = "output.txt";
    let content = "Hello, World!";

    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    println!("Content written to file: {}", file_path);

    Ok(())
}
