fn print_bmi(height: f32, weight: Option<f32>) {
    // Calc BMI with the option type.
    let bmi:Option<f32> = match weight {
        Some(w) => Some(w/(height/100.0).powf(2.0)),
        None => None,
    };

    let msg = match bmi {
        Some(b) if b < 18.5 => "Underweight",
        Some(b) if b < 23.0 => "Normal",
        Some(b) if b < 25.0 => "Overweight",
        Some(b) if b < 30.0 => "Obsese Class 1",
        Some(b) if b < 35.0 => "Obese Class 2",
        Some(_) => "Obese Class 3",
        None => "Unknown", 
    };

    println!("BMI: {:?}, Message: {}", bmi, msg);
}

fn main(){
    let height = 170.0;
    print_bmi(height, Some(60.0));
    print_bmi(height, Some(80.0));
    print_bmi(height, None);

}