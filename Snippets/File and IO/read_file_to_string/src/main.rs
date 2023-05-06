use std::fs;

fn main() -> std::io::Result<()> {
    let file_path = "example.txt";
    let content = fs::read_to_string(file_path)?;

    println!("File content:\n{}", content);

    Ok(())
}
