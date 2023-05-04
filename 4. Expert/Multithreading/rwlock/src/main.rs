use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let read_handle = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            let data_read = data.read().unwrap();
            println!("Reader: {:?}", *data_read);
        }
    });

    let write_handle = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            let mut data_write = data.write().unwrap();
            data_write.push(4);
            println!("Writer: {:?}", *data_write);
        }
    });

    read_handle.join().unwrap();
    write_handle.join().unwrap();
}
