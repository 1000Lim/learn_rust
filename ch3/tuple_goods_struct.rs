struct Item(String, f64);

fn main() {
    let banana = Item("Banana".to_string(), 1.0);
    let apple = Item("Apple".to_string(), 2.0);
    let orange = Item("Orange".to_string(), 3.5);

    let items = vec![banana, apple, orange];
    let total: f64 = print_items_and_get_sum(&items);
    println!("Total: {}", total);
}

fn print_item(item: &Item) {
    println!(
        "Item: {name}, Price: {price}",
        name = item.0,
        price = item.1
    );
}

fn print_items_and_get_sum(items: &Vec<Item>) -> f64 {
    let mut total = 0.0;
    for item in items {
        print_item(&item);
        total += item.1;
    }
    total
}
