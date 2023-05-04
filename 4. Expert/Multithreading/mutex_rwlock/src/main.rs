use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));
    let mut tasks = vec![];

    for _ in 0..10 {
        let data = Arc::clone(&data);
        let task = tokio::spawn(async move {
            let mut data = data.lock().await;
            *data += 1;
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    println!("Result: {}", *data.lock().await);
}
