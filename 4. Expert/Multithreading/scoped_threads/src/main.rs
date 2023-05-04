use crossbeam::scope;

fn main() {
    let data = [1, 2, 3, 4, 5];

    scope(|s| {
        let handle = s.spawn(|_| {
            println!("Data: {:?}", &data);
        });

        handle.join().unwrap();
    }).unwrap();
}
