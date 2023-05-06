fn main() {
    let s1 = "hello"; // &str
    let s2 = String::from("world"); // String

    let s3 = format!("{} {}", s1, s2);
    println!("{}", s3);
}
