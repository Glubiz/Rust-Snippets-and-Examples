use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    let handle = tokio::spawn(async move {
        let msg = String::from("Hello");
        tx.send(msg).await.unwrap();
    });

    let received = rx.recv().await.unwrap();
    println!("Received: {}", received);

    handle.await.unwrap();
}