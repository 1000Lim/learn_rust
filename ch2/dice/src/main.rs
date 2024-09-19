use rand::Rng;
fn main() {
    let mut rng_number = rand::thread_rng();
    for _ in 0..5 {
        let random_number = rng_number.gen_range(1..=6); // Generates number between 1 and 6
        println!("Dice number: {}", random_number);
    }
}
