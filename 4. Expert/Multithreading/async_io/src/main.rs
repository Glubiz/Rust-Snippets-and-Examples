use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::open("file.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("Contents: {}", contents);
    Ok(())
}
