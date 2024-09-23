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


## Iterator
Iterator Method.
| Method        | Description                           |
|---            |---                                    |
|iter           | No ownership transfer return (&T)     |
|iter_mut       | No ownership transfer return (&mut T) |
|into_iter      | Ownership transfer (T)                |

## Iterator Trait
- Iterator Trait Syntax:
    ```rust
    trait Iterator{
        type Item;
        fn next(&mut self) -> Option<Self::Item>;

    }
    ```
- Interator Trait Example Code: [iter_fib.rs](./iter_fib.rs)

## Enum and Pattern Matching
### Syntax for the enum type (without type)
```rust
// Define trait_name
enum enum_name {
    value1, 
    value2, 
    value3,
    //...
}

// User the defined enum
let variable1 = enum_name::value1;
let variable2 = enum_name::value2;
```

### Syntax for the enum (with type)
```rust
enum enum_name {
    value1(data_type),
    value2(data_type),
    value3(data_type),
    //...
}
```

### Pattern Matching
- Normal Match Example
    ```rust
    // fizzbuzz code
    match (i%3, i%5) {
        (0, 0) => println!("FizzBuzz"),
        (0, _) => println!("Fizz"),
        (_, 0) => println!("Buzz"),
        _ => println!("{}", i),  // otherwise, print number.
    }
    ```
- __Match Guard__: add `if` after condition matching
    ```rust
    let msg = number match {
        n if n % 15 == 0 => "FizzBuzz".to_string(),
        n if n % 3 == 0 => "Fizz".to_string(),
        n if n % 5 == 0 => "Buzz".to_string(),
        _ => format!("{}", i), // use format to make a format string.
    }
    ```
    - further example: [match_bmi.rs](./match_bmi.rs)

## Rust Module, Crate and Package

## Module (One Scope)
Unit of Scope: A module is a way to organize code within a crate. Modules allow you to group related functions, structs, enums, constants, and other items into a namespace, helping to prevent name collisions and making the codebase more manageable.

|   Description         | Example                       |
|   ---                 |   ---                         |
| From the top          | use crate::aaa::bbb;          |
| From the parent       | use super::eee;               |
| Use the alias         | use aaa::bb::print as b_print;|
| Use multiple modules  | use aaa::{bbb, ccc};          |
| Use all from module   | use aaa::*;

- module example code: [rand](./rand/src/main.rs) package.

### Crate (Represents Tree structure of module)
A crate is the smallest compilation unit in Rust.
- Binary Crate: Generates an executable program. It must have a main function(`main.rs`).
- Library Crate: Generates a reusable library, which other crates can depend on(`lib.rs`).


## Package (Multiple Crates)
A package is a collection of one or more crates that are managed together using Cargo, Rust's build system and package manager. A package is defined by a Cargo.toml file, which specifies metadata, dependencies, and other configuration options.