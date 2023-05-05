use once_cell::sync::OnceCell;

static CELL: OnceCell<String> = OnceCell::new();

fn main() {
    let _ = CELL.set(String::from("Hello, world!"));

    if let Some(value) = CELL.get() {
        println!("Value: {}", value);
    } else {
        println!("Value not set");
    }
}
