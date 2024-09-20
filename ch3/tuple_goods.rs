fn main() {
    let banana = ("Banana", 2.5);
    let apple = ("Apple", 3.5);
    let orange = ("Orange", 4.5);
    print_fruit(banana);
    print_fruit(apple);
    print_fruit(orange);
    let total = banana.1 + apple.1 + orange.1;
    println!("Total price: {}", total);
}

fn print_fruit(fruit: (&str, f64)) {
    println!(
        "Fruit: {name}, Price: {price}",
        name = fruit.0,
        price = fruit.1
    )
}
