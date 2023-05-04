trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("i32: {}", self);
    }
}

impl Printable for f64 {
    fn print(&self) {
        println!("f64: {}", self);
    }
}

fn display<T: Printable>(value: T) {
    value.print();
}

fn main() {
    let a: i32 = 42;
    let b: f64 = 3.14;

    display(a);
    display(b);
}
