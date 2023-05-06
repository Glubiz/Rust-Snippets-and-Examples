use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&person).unwrap();
    println!("JSON: {}", json);

    // Deserialize from JSON
    let deserialized_person: Person = serde_json::from_str(&json).unwrap();
    println!("Deserialized: {:?}", deserialized_person);
}
