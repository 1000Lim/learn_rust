use std::{env, fs};

fn main() {
    let args = env::args();
    let mut total = 0.0;

    if args.len() < 2 {
        println!("Please input a file name");
        return;
    }

    for (i, s) in args.enumerate() {
        if i == 0 {
            continue;
        }

        let contents = match fs::read_to_string(s) {
            Ok(c) => c,
            Err(e) => {
                println!("Error reading file: {}", e);
                continue;
            }
        };

        let lines = contents.split("\n");
        for line in lines {
            let number: f64 = match line.parse() {
                Ok(n) => n,
                Err(_) => 0.0,
            };
            total += number;
        }
    }

    println!("Total: {}", total);
}
