use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut output = File::create("output.txt")?;
    output.write_all(contents.as_bytes())?;

    Ok(())
}
