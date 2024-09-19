use std::collections::HashMap; // 1. import hanshmap module.

const VOTING_DATA: &str = "A, B, C, A, A, A, B, B, C";

fn main() {
    let mut counter_map = HashMap::new(); // 2. Create a new hashmap.

    // 3. Initialize the hashmap with the keys and values.
    counter_map.insert("A", 0);
    counter_map.insert("B", 0);
    counter_map.insert("C", 0);

    // 4. Split the string by comma and iterate over the elements.
    for key in VOTING_DATA.split(", ") {
        counter_map.insert(key, counter_map[key] + 1);
    }

    for key in ["A", "B", "C"] {
        println!("{}: {}", key, counter_map[key]);
    }
}
