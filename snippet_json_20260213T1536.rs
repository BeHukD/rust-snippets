// snippet_json_20260213T1536.rs
// Topic: JSON serialization and deserialization with serde
// Demonstrates deriving Serialize/Deserialize and working with JSON data.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Company {
    name: String,
    employees: Vec<Person>,
    departments: HashMap<String, Vec<String>>,
}

fn main() {
    // Serialize a Person struct to JSON string
    let alice = Person {
        name: "Alice".to_string(),
        age: 30,
        is_active: true,
        email: Some("alice@example.com".to_string()),
    };

    let json_alice = serde_json::to_string_pretty(&alice).unwrap();
    println!("Serialized Person:\n{}", json_alice);

    // Deserialize JSON string back to Person
    let deserialized: Person = serde_json::from_str(&json_alice).unwrap();
    println!("Deserialized back: {:?}", deserialized);

    // More complex example: Company with nested structures
    let bob = Person {
        name: "Bob".to_string(),
        age: 25,
        is_active: false,
        email: None,
    };

    let mut departments = HashMap::new();
    departments.insert("Engineering".to_string(), vec!["Alice".to_string(), "Bob".to_string()]);
    departments.insert("HR".to_string(), vec!["Eve".to_string()]);

    let company = Company {
        name: "Acme Corp".to_string(),
        employees: vec![alice, bob],
        departments,
    };

    let json_company = serde_json::to_string_pretty(&company).unwrap();
    println!("\nSerialized Company:\n{}", json_company);

    // Deserialize the company
    let _company_back: Company = serde_json::from_str(&json_company).unwrap();
    println!("Deserialized Company successfully");

    // Bonus: serde_json::Value for dynamic JSON
    let raw_json = r#"{"title":"Serde is great","stars":42,"tags":["rust","json"]}"#;
    let value: serde_json::Value = serde_json::from_str(raw_json).unwrap();
    println!("\nDynamic JSON value: {:?}", value);
    println!("Title: {}", value["title"]);
}

/*
Dependencies for Cargo.toml:

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

Run with:
cargo run --bin snippet_json_20260213T1536.rs
*/
