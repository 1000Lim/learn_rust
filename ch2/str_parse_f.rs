fn main() {
    let s = "123.456";
    // Parsing string to float
    let number = s.parse::<f64>().expect("Please input a correct number.");
    println!("Number: {}", number);
}
