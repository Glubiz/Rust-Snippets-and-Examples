use crossbeam::queue::SegQueue;
use std::sync::Arc;
use std::thread;

fn main() {
    let queue = Arc::new(SegQueue::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            queue.push(1);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let sum: i32 = queue.iter().sum();
    println!("Sum: {}", sum);
}
