struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

fn main() {
    let c = Circle { radius: 5.0 };
    println!("Circle area: {}", c.area());
}
