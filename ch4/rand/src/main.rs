mod random; // import the random module
use crate::random:: {linear, xorshift}; // import the linear and xorshift modules

fn main() {
    let mut seed = 1u32;
    let r1 = linear::rand(&mut seed);
    let r2 = xorshift::rand(&mut seed);
    println!("r1: {}, r2: {}", r1, r2);
}
