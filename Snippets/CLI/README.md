# Command Line Interfaces (CLI) in Rust

This folder contains code snippets and explanations for creating Command Line Interface (CLI) applications using Rust. CLIs are text-based programs that interact with users through a command prompt or terminal, allowing them to enter commands and receive textual feedback. In this folder, you will find two CLI examples: a factorial calculator and a list sorter.

## Factorial Calculator

The factorial calculator is a simple CLI application that calculates the factorial of a given non-negative integer. Factorial of a non-negative integer `n` is the product of all positive integers less than or equal to `n`. It is denoted by `n!`. The program accepts an integer as input and returns the calculated factorial.

## List Sorter

The list sorter is another CLI application that takes a list of integers or strings as input and sorts them in either ascending or descending order. The program accepts a list of elements and an optional sorting order flag, and then returns the sorted list.

## Creating CLI Applications in Rust

Creating CLI applications in Rust is straightforward, as the language provides built-in functionality for parsing command line arguments and handling I/O. Here are some steps to create a simple CLI application:

1. Parse command line arguments using the `std::env::args()` function, which returns an iterator over the command line arguments.
2. Define the core functionality of the application as separate functions or modules.
3. Process the input, call the appropriate functions, and provide user feedback through the command line.
4. Handle errors gracefully by using the `Result` type and providing meaningful error messages.

## Learning Resources

To learn more about creating CLI applications in Rust, you can explore the following resources:

1. [The Rust Programming Language Book - Chapter 12: An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
2. [Rust by Example - Command Line Arguments](https://doc.rust-lang.org/rust-by-example/std_misc/arg.html)
3. [Building a CLI Application in Rust](https://www.jamestharpe.com/rust-cli/)

Feel free to explore the code snippets in this folder and use them as a starting point for creating your own CLI applications in Rust.
