use std::io::{stdin, Write, BufRead, BufReader};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn start_thread(socket: TcpStream){
    let mut reader = BufReader::new(socket);
    thread::spawn(move || {
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf){
            if n > 0 {
                println!("Recieved {}", buf.trim());
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

fn input(msg: &str) -> String{
    if msg != "" {
        print!("{}", msg);
    };
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Input error");
    String::from(buf.trim())
}

fn main() {
    let server_addr = "127.0.0.1:8888";

    // Connect to the server
    let mut socket = TcpStream::connect(server_addr).expect("Failed to connect to server");
    socket.set_nonblocking(true).expect("Failed to set non-blocking mode");
    println!("Connected to server at {}", server_addr);
    start_thread(socket.try_clone().expect("Failed to clone socket"));

    let user = input("Enter your name: ");
    println!("Welcome, {}!", user);

    loop {
        let msg = input("Enter your message: ");
        let msg = format!("{} > {}\n", user, msg);
        let buf = msg.as_bytes();
        socket.write_all(buf).unwrap();
    }
}
