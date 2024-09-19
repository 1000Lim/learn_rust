use std::collections::HashMap;

fn main() {
    let mut months = [
        "해오름달",
        "시샘달",
        "꽃내음달",
        "잎새달",
        "푸른달",
        "누리달",
        "빗방울달",
        "타오름달",
        "거둠달",
        "온누리달",
        "눈마중달",
        "매듭달",
    ];

    // Intialize the months
    let mut months_map = HashMap::new();
    for (i, v) in months.iter().enumerate() {
        months_map.insert(i + 1, v);
    }

    // Print the months
    for (k, v) in &months_map {
        println!("{}: {}", k, v);
    }
}
