struct BmiRange {
    min: f64,
    max: f64,
    label: String,
}

impl BmiRange {
    fn new(min: f64, max: f64, label: &str) -> Self {
        BmiRange {
            min: min,
            max: max,
            label: label.to_string(),
        }
    }
    fn in_range(&self, v: f64) -> bool {
        v >= self.min && v < self.max
    }
}

struct Body {
    height: f64,
    weight: f64,
    name: String,
}

impl Body {
    fn new(height: f64, weight: f64, name: &str) -> Self {
        Body {
            height: height,
            weight: weight,
            name: name.to_string(),
        }
    }

    fn calc_bmi(&self) -> f64 {
        self.weight / (self.height / 100.0).powi(2)
    }

    fn print_result(&self) {
        let bmi = self.calc_bmi();
        let mut result = String::new();

        let ranges = vec![
            BmiRange::new(0.0, 18.5, "UnderWeight"),
            BmiRange::new(18.5, 23.0, "Normal"),
            BmiRange::new(23.0, 25.0, "OverWeight"),
            BmiRange::new(25.0, 30.0, "Fat Level 1"),
            BmiRange::new(30.0, 35.0, "Fat Level 2"),
            BmiRange::new(35.0, 40.0, "Fat Level 3"),
            BmiRange::new(40.0, f64::INFINITY, "Not Human"),
        ];

        for range in ranges {
            if range.in_range(bmi) {
                result = range.label.clone();
                break;
            }
        }

        println!("{}'s BMI: {}, Result: {}", self.name, bmi, result);
    }
}

fn main() {
    let hong = Body::new(165.0, 80.0, "Hong");
    let lim = Body::new(170.0, 65.0, "Lim");
    let kim = Body::new(180.0, 200.0, "Kim");

    hong.print_result();
    lim.print_result();
    kim.print_result();
}
