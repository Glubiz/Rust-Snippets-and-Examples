use futures::{stream, StreamExt};

#[tokio::main]
async fn main() {
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);

    let sum = stream.fold(0, |acc, x| async move { acc + x }).await;
    println!("Sum: {}", sum);
}
