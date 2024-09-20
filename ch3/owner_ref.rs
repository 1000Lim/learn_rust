fn main() {
    let g1 = String::from("Original Sentence");
    show_message(&g1);
    println!("g1: {}", g1);
}

fn show_message(message: &String) {
    println!("Message called from function: {}", message);
}
