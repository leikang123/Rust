// thread_basic.rs
use std::thread;

fn main() {
    thread::spawn( || {
        println!("Thread!");
        "Much concurent,such wow!".to_string()

    });
    println!("Hello, world!");
}
