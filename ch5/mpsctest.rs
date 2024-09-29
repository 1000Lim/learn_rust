use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn sleep_sender(name: &str, sender: mpsc::Sender<String>){
    let whales = vec!["beluga", "orca", "humpback", "blue", "sperm"];
    for whale in whales {
        sender.send(format!("{}:{}", name, whale)).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
    sender.send("quit".to_string()).unwrap();
}

fn main() {
    println!("The multi-producer, single-consumer channel test");
    let (tx, rx) = mpsc::channel::<String>();

    // multiple senders
    let sender = tx.clone();
    thread::spawn(move || {
        sleep_sender("Thread 1", sender)
    });
    let sender = tx.clone();
    thread::spawn(move || {
        sleep_sender("Thread 2", sender)
    });

    // single receiver
    for received in rx {
        println!("Received: {}", received);
        if received == "quit" {
            break;
        }
    }

}