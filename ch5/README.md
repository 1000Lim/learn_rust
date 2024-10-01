## RUST exercise
### Image Processing
#### Image crop and resizing
```rust
// crop the original image to the x, y, w, z.
let img = imageops::crop(&mut img, x, y, w, z).to_image();
let img2 = imageops::crop(&mut imag, heigh, length, filter).to_image()
```
- Example Code: [image_thumb](./image_thumb/)

## Thread Handling
- Sleeping Thread
    ```rust
    use std::{thread, time};

    // sleep for one second.
    thread::sleep(time::Duration::from_millis(1000));
    ```

    - Spawn new thread
    ```rust
    thread::spawn(|| {func_run_by_thread})
    ```
    - Example code: [threadtest.rs](./threadtest.rs)

- Sharing data via threads.
    __MPSC channel__ stands for multi-producder, single-consumer channel. This mechanism allows multiple threads (producers) to send messages or data to one thread (consumer) safely and efficiently.
    - Multiple Producers: Any number of producer threads can send data into the channel.
    - Single Consumer: Only one consumer thread receives and processes  the data from the channel.
    - Thread-Safe Communication: Ensures that data sent between thread is managed without race conditions or data corruption.
    - Message Passing: Data is sent as messages, which can be any type of  data structure.
    ```rust
    use std::sync::mpsc;
    ```

- Startup non-blocking server.
```rust
let server = TcpListener::bind("127.0.0.1:8888").unwrap(); // bind address to the TCP Listener.
// non-blocking mode.
server.set_nonblocking(true).unwrap();

// matching server Ok or Error.
match server.accept(){
    Ok((socket, addr)) => println!("Connection success:  {:?}", addr),
    Err(e) => println!("Failed to access server: {:?},", e)
}
```


Rust Backend frameworkd
- [__Actix Web__](https://actix.rs/docs/application) : The most mature and widely used framework in the Rust.
    - Key features:
        - High performance and low-latency.
        - Asynchronous (based on tokio).
        - Support for both HTTP/1 and HTTP/2.
        - Middleware support (e.g., logging, security, etc.).
        - WebSocket support.
        - Strong ecosystem with integrations for databases, authentication, and more.
    - Example Code: [web_hello](./web_hello/)

-  [__Axum__](./https://docs.rs/axum/latest/axum/) Axum is a relatively newer framework compared to Actix Web, but it has quickly gained popularity due to its simplicity, ease of use, and deep integration with the tokio async runtime and hyper (Rust's core HTTP library).
    - Key Features:
        - Asynchronous (built on tokio and hyper).
        - Simple and declarative routing system.
        - Middleware support.
        - Support for WebSockets.
        - Extensible via tower, a set of composable middleware and services.
        - Tight integration with tokio, hyper, and tower.
        - Usage: Axum is often used for building APIs and microservices with a focus on ergonomics and developer productivity.
    - Example code: [axum_hello](./axum_hello/)
