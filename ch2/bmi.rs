use std::io;

fn bmi(mut height: f64, weight: f64) -> f64 {
    height = height / 100.0;
    weight / height.powf(2.0)
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    // Read the input from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // trim the string and convert string to float and
    input.trim().parse().expect("Please type a number")
}

fn main() {
    // This module is made for calculating BMI.
    let height_cm = input("Enter your height in cm: ");
    let weight_kg = input("Enter your weight in kg: ");

    println!("Your BMI is: {}", bmi(height_cm, weight_kg));
}
