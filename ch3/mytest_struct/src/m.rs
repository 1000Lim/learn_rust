pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// PartialEq allows us to compare two instances of GItem
#[derive(Debug, PartialEq)] // Derive the PartialEq and Debug traits
struct GItem {
    name: String,
    price: i64,
}

#[cfg(test)]
mod tests {
    use super::*; // Import names from outer scope

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_gitem() {
        let item1 = GItem {
            name: "item1".to_string(),
            price: 1000,
        };
        let mut item2 = GItem {
            name: "item2".to_string(),
            price: 2000,
        };
        assert_ne!(item1, item2);

        // change the item2's name and price.
        item2.name = "item1".to_string();
        item2.price = 1000;
        assert_eq!(item1, item2);
    }
}
