fn main() {
    let s = "123.456s";
    let num = s.parse::<f64>();
    match num {
        Ok(number) => println!("Parsed number: {}", number),
        Err(e) => println!("Please input the correct number e: {}", e),
    }
}
