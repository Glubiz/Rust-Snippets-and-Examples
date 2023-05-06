use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let msg = String::from("Hello");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);

    handle.join().unwrap();
}
