# Chapter 4: Structure and method

## Structure and Method
Structure of the sturcture and method.
```rust
struct stucture_name{
    // define field.
}

impl structure_name{
    fn method_1(&self, param1, param2...){
        // define method 1.
    }

    fn method_2(&self, param1, param2...){
        // define method 2.
    }
}
```
- example code: [bmi_chekers.rs](./bmi_checkers.rs)

### Clone and rewrite structure
- use ..`structure_name`
    ``` rust
    let alex = Person::new("Alex", 19);
    let tim = Person::new("Tim", ..alex) // copy alex's other arguments to Tim.
    ```
- use `#[derive(Clone)]`
    ```rust
    #[derive(Clone)] // can use clone method.
    struct Person{
        name: String,
        age: i32,
    }

    ...
    let andrew = Person("Andrew", 20);
    let mut betty = andrew.clone();
    betty.name = "betty"
    ```
## Trait
Trait is a collection of methods (or method signatures) that define _shared behavior for types_. Traits are similar to interfaces in other languages, as they allow you to define functionality that multiple types can implement. They enable Rust's form of _polymorphism_.

__Trait Syntax__
```rust
// define trait
trait trait_name{
    fn method_1(&self, param1, param2 ...) -> return_type;
    fn method_2(&self, param1, param2 ...) -> return_type;
}

// impleemnt trait
impl trait_name{
    fn method_1(&self, param1, param2...) -> return_type {
        // process method.
    }
}
```
- example_code: [trait_trasurebox.rs](./trait_trasurebox.rs)

## Generic
Generic Features:
- Generics allow for abstract, reusable code that works with different types while maintaining compile-time type safety.
- Use generics in functions, structs, enums, and traits.
- Rust's generics are monomorphized, meaning that they have no runtime performance overhead.
- Trait bounds allow us to restrict the types that can be used with generics, ensuring they meet specific criteria (e.g., implementing certain traits).
    ```rust
    // print_largest should've implment the std::fmt::Display trait.
    fn print_largest<T: PartialOrd + std::fmt::Display>(list: &[T]) {
        let largest = largest(list);
        println!("The largest is: {}", largest);
    }
    ```

Example code: [structure_method_generics.rs](./structure_method_generics.rs)