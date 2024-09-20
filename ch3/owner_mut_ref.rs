fn main() {
    let mut s1 = String::from("Hello, world");
    println!("s1: {}", s1);
    add_quote(&mut s1);
    println!("s1: {}", s1);
}

fn add_quote(s: &mut String) {
    s.insert(0, '"');
    s.push('"');
}
