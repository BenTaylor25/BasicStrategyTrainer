use std::io;
use std::io::{stdin, Write};

fn main() {
    println!("Hello, world!");

    print!("Enter name: ");
    io::stdout().flush().unwrap();

    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    println!("Hi {}", input_string);
}
