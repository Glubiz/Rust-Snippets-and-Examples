use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::task;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(2)); // limit concurrent access to 2

    let tasks = (1..6).map(|n| {
        let semaphore = Arc::clone(&semaphore);
        task::spawn(async move {
            let permit = semaphore.acquire().await;
            println!("Task {} acquired permit", n);
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Task {} released permit", n);
            drop(permit);
        })
    }).collect::<Vec<_>>();

    futures::future::join_all(tasks).await;
}
