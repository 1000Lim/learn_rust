use rand::{seq::SliceRandom, Rng};

fn main() {
    println!("Hello, Bingo Card.");
    let mut nums = [0; 75]; // fill with 0 in 75 elements.
    for i in 1..=75 {
        nums[i - 1] = i;
    }

    let mut rng = rand::thread_rng(); // random number generator.
    nums.shuffle(&mut rng); // shuffle the numbers.

    for i in 0..5 {
        for j in 0..5 {
            let x = i * 5 + j;
            if x == 12 {
                print!("  *,"); // place wildcard in the center.
            } else {
                print!("{:3},", nums[x]);
            }
        }
        println!();
    }
}
