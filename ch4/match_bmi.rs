fn print_bmi(height: f32, weight: Option<f32>) {
    // Calc BMI with the option type.
    let bmi::Option<f32> = match weight {
        Some(w) => Some(w/(height/100.0).powf(2.0)),
        None => None,
    };

    println!("hello");
}

fn main(){
    println!("Calc BMI.");
}