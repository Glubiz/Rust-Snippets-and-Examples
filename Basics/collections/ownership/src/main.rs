// Example 1: Ownership
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("{}", s1); // This would cause an error
    println!("{}", s2); // This is valid
}

// Example 2: Ownership
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Immutable borrow
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
