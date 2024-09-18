fn main() {
    for number in 1..=50 {
        if number.to_string().contains('3') || number % 3 == 0 {
            println!("A");
        } else {
            println!("{}", number);
        }
    }
}
