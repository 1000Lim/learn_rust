use std::fs::{self, File};
use std::io::{BufWriter, Write};

fn main() {
    let filename = "fizzbuzz.txt";

    // Open the file and write fizzbuzz to it
    {
        let file = match File::create(filename) {
            Ok(f) => f,
            Err(e) => {
                println!("Error creating file: {}", e);
                return;
            }
        };

        let mut writer = BufWriter::new(&file);

        for i in 1..=100 {
            match (i % 3, i % 5) {
                (0, 0) => writeln!(writer, "FizzBuzz").unwrap(),
                (0, _) => writeln!(writer, "Fizz").unwrap(),
                (_, 0) => writeln!(writer, "Buzz").unwrap(),
                (_, _) => writeln!(writer, "{}", i).unwrap(),
            }
        }
    }

    // read the file and print the contents
    let contents = match fs::read_to_string(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    println!("Contents of the file:{}", contents);
}
