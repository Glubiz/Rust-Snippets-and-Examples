use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Error: Please provide a single non-negative integer as a command-line argument.");
        process::exit(1);
    }

    let n: u64 = match args[0].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid input. Please provide a non-negative integer.");
            process::exit(1);
        }
    };

    let result = factorial(n);
    println!("{}! = {}", n, result);
}

fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}
