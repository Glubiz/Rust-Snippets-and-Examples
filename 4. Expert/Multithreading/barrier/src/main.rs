use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    for _ in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Before the barrier");
            barrier.wait();
            println!("After the barrier");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
