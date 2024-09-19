# Cargo

## Cargo Feature

| Cargo Feature     | Command   |
|---                |---        |
| Create Project    | cargo `new` _project_name_    |
| Build Project     | cargo `build`                 |
| Run Project       | cargo `run`                   |
| Check Syntax      | cargo `check`                 |
| Run Test Codes    | cargo `test`                  |
| Generate Documents| cargo `doc`                   |
| Publish library   | cargo `publish`               |

## How to use the external library
- Add dependancy under the `Cargo.toml` file's `[dependencies]`.
    ```toml
    [dependencies]
    num-bigint = "0.4"
    ```
- From the source code import the library.
    - `use crate_name::module`
    - `use crate_name::module1::module1-1`
    - `use crate_name:: {moduleA, moduleB}`


## How to parse?
- Example Code: [str_parse_f.rs](./str_parse_f.rs)
    ```rust
    // convert the string to float example.
    let number = s.parse::<f64>.expect("Converting Failed."); // or we can use unwrap() instead.
    ```
- Exception Handling Code (Reuslt) [parse_result_exception_handling.rs](./parse_result_exception_handling.rs)
    ```rust
    let number = s.parse::<f64>();
    // Handling Result type.
    match number {
        Ok(Result) => println!("Your number is {}", Result),
        Err(e) => println("Please input the correct number e: {}", e)
    }
    ```

## Type Name
- Value declration with type.
```rust
let a = 100u8; // unsigned int 100
let b = 100i128 // i128 100
let c = 10_000 // 10000
```


## HashMap
expression: `let mut variable_name: HashMap<KeyType, ValueType> = HashMap::new()`

- Example Code (Type Inference): [type_inference_voting_counter.rs](./hashmap/type_inference_voting_counter.rs)
- Example Code (Explict Typing): [explicit_typing_korean_months.rs](./hashmap/explicit_typing_korean_months.rs)
- Example Code (`Option` Handling):[option_unexists_keys.rs](./hashmap/option_unexists_key.rs)
    ```rust
    let value = map.get(key);
    match value {
        Some(v) => println!("The value of key {} is {}", key, v);
        None => println!("The key {} does not exists in map.", key);
    }
    ```
