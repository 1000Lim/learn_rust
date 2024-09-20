struct Body {
    weight: f64, // struct field seperated by comma
    height: f64,
} // struct ends with a curly brace without a semicolon

fn calc_bmi(body: &Body) -> f64 {
    body.weight / (body.height / 100.0).powi(2)
}

fn main() {
    let hong = Body {
        weight: 80.0,
        height: 165.0,
    };

    let lim = Body {
        weight: 65.0,
        height: 170.0,
    };

    println!("Hong's BMI: {}", calc_bmi(&hong));
    println!("Lim's BMI: {}", calc_bmi(&lim));
}
