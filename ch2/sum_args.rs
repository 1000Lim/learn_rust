fn main() {
    let args = std::env::args();
    let mut total = 0.0;

    for (i, s) in args.enumerate() {
        if i == 0 {
            continue;
        }
        let number: f64 = match s.parse() {
            Ok(n) => n,
            Err(e) => {
                println!("Please input a correct number: {}", e);
                0.0
            }
        };
        total += number;
    }
    println!("Total: {}", total);
}
