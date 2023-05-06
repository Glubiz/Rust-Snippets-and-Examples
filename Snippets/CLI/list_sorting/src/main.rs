use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Error: Please provide a list of integers as command-line arguments.");
        process::exit(1);
    }

    let mut numbers: Vec<i32> = match args.into_iter().map(|arg| arg.parse::<i32>()).collect() {
        Ok(numbers) => numbers,
        Err(_) => {
            eprintln!("Error: Invalid input. Please provide a list of integers.");
            process::exit(1);
        }
    };

    numbers.sort();
    println!("Sorted numbers: {:?}", numbers);
}
