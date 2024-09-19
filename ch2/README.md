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


## How to pars?
- Example Code: [str_parse_f.rs](./str_parse_f.rs)
    ```rust
    // convert the string to float example.
    let number = s.parse::<f64>.expect("Converting Failed.");
    ```
