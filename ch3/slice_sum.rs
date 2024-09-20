// The function add sum of the sliced value.
fn sum_slice(items: &[i64]) -> i64 {
    let mut total = 0;
    for item in items {
        total += item;
    }
    total
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let total = sum_slice(&arr[..]);
    println!("Total: {}", total);

    let vector = vec![1, 2, 3, 4, 5];
    let total = sum_slice(&vector[2..]);
    println!("Total: {}", total);
}
