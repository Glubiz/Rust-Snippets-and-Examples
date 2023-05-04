use std::collections::{HashMap, HashSet};

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    if let Some(value) = map.get("key1") {
        println!("Value for key1: {}", value);
    }

    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    if set.contains(&2) {
        println!("Set contains 2");
    }
}
