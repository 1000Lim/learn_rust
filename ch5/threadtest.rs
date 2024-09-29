use std::{thread, time};

fn sleep_print(world: &str, count: i32) {
    for i in 1..=count {
        println!("{}: {}", world, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main(){
    sleep_print("Start Main", 5);
    thread::spawn(|| {
        sleep_print("Thread 1", 3);
    });
    thread::spawn(|| {
        sleep_print("Thread 2", 5);
    });
    sleep_print("End Main", 5);
}