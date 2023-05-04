use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let value = map.get("two");
    println!("Value of 'two': {:?}", value);
}
