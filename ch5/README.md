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