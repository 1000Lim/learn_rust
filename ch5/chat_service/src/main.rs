use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn start_thread(client: TcpStream, tx: mpsc::Sender<String>){
    let mut reader = BufReader::new(client);
    thread::spawn(move || {
        let mut msg =  String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            if n > 0 {
                tx.send(msg).expect("Failed to send message");
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}

fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];
    for mut socket in clients.into_iter() {
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes){
            println!("Failed to send message: {}", e);
            continue;
        }
        collector.push(socket);
    }
    collector
}

fn main() {
    let server_addr = "127.0.0.1:8888";
    let (tx, rx) = mpsc::channel::<String>();
    let mut clients: Vec<TcpStream> = Vec::new();

    // start server
    let server = TcpListener::bind(server_addr).expect("Failed to bind server");
    server.set_nonblocking(true).expect("Failed to set nonblocking");
    println!("Server started at {}", server_addr);

    // start receiver
    loop {
        // client connection
        if let Ok((client, addr)) = server.accept() {
            println!("New client connected: {}", addr);
            clients.push(client.try_clone().expect("Failed to clone client"));
            start_thread(client, tx.clone());
        }

        if let Ok(msg) = rx.try_recv() {
            println!("Received message: {}", msg.trim());
            clients = send_all(clients, &msg);
        }

        thread::sleep(Duration::from_millis(100));
    }

}
