// The code implements fibonacci series.
fn main() {
    let (mut a, mut b) = (1, 1);
    for i in 1..=30 {
        println!("{}: {}", i, a);
        let sum = a + b;
        a = b;
        b = sum;
    }
}
