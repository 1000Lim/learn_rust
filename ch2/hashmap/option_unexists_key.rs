use std::collections::HashMap;

fn main() {
    println!("An example for handling option unexists key");

    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);

    let key = "three";
    let value = map.get(key);

    match value {
        Some(v) => println!("The value of key {} is {}", key, v);
        None => println!("The key {} does not exists in map.", key);
    }
}
