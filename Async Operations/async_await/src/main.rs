use futures::executor::block_on;

async fn hello_async() {
    println!("Hello, async world!");
}

fn main() {
    let future = hello_async();
    block_on(future);
}
