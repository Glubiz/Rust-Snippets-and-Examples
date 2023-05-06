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

    // Serialize to YAML
    let yaml = serde_yaml::to_string(&person).unwrap();
    println!("YAML: {}", yaml);

    // Deserialize from YAML
    let deserialized_person: Person = serde_yaml::from_str(&yaml).unwrap();
    println!("Deserialized: {:?}", deserialized_person);
}
