use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected to the server");

    let message = b"Hello, World!";
    stream.write(message).unwrap();

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    println!(
        "Received from server: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );
}
