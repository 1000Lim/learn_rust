# Chapter3: Ownership in Rust
In Rust, the ownership system refers to the set of rules that govern how memory is managed, ensuring memory safety without needing a garbage collector. Rust's ownership system revolves around three key principles:

## Ownership
- __Ownership Principles__
    - Each value in Rust has a variable that is its owner.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value is automatically dropped, freeing the memory.
- Some value don't use the ownership: intenger, float, boolean and others that usually stored in stack.
- Example code: [owener_free.rs](./owener_free.rs)

## Borrowing
Borrow a reference to a value. Borrowing allows you to use data without taking ownership.
- __Immutable borrowing__ (`&T`): You can have multiple immutable references to a value at the same time.
    - Example Code: [owner_ref](./owner_ref.rs)
- __Mutable borrowing__ (`&mut T`): You can have only one mutable reference to a value at a time to avoid data races.
    - Example Code: [owner_mut_ref.rs](./owner_mut_ref.rs)

- __Lifetimes__: Lifetimes ensure that references are valid as long as the data they point to is still valid. This prevents dangling references or use-after-free errors. Lifetimes are often inferred by the compiler, but they can be explicitly annotated in more complex cases.

## Tuple, Array and Slice
- __Tuple__: To use the tuple, we can easily gather the variables(ex: [tuple_goods.rs](./tuple_goods.rs)).
    ```rust
    let var = (var1, var2);
    println!("{}, {}", var.0, var.1);
    ```
    - Define Structure with Tuple: `struct {struct_name} ({type1}, {type2}...)`(ex: [tuple_goods_struct.rs](./tuple_goods_struct.rs))
        - Normal Structure Example: [struct_bmi.rs](./struct_bmi.rs)

- __Slice__: Refer the parts of the value(from variable_name n to m index: `&variable_name[n..m]` ex: )

## String Type: String and &str
- String and &str
    - `String`: Owned, heap-allocated, mutable, growable string.
    - `&str`: Borrowed, immutable reference to a string slice, more lightweight, but cannot be modified.

- Difference with C/C++
    - C ends with `'\0'`, mark as ends of string.
    - Rust store lenght and capacity.

- Example:
    - Str Shadowing: Keep the origianl string as immutable.
        ```rust
        fn main() {
            let s = "Rust is fantastic!";
            let s = s.replace("fantastic", "great"); // shadowing orignal value.
            println!("{}", s);
        }
        ```

## Cargo Test
Example: [my_struct/src/lib.rs](./mytest_struct/src/lib.rs)
- To compare the struct use `PartialEq`
    ```rust
    #[derive(Debug, PartialEq)] // Derive the PartialEq and Debug traits
    struct GItem {
        name: String,
        price: i64,
    }
    ...
    #[test] // Indicates this is the test function.
    fn test_gitem() {
        ...
        assert_eq!(item1, item2); // assert two item is equal.
        assert_ne!(item1, item2); // assert two item is not eaqual.
    }
    ```