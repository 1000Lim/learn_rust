use num_bigint::BigInt;
fn main() {
    let value = BigInt::from(1234);
    println!("value: {}", value.pow(5678));
}
