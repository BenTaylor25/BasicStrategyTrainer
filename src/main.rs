use std::io::{self, stdin, Write};

fn int_input(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    let int = input_string.trim().parse()
        .expect("Invalid number");

    int
}

fn main() {
    println!("Hello, world!");

    let a = int_input("Enter first number: ");
    let b = int_input("Enter second number: ");

    let c = a + b;

    println!("{}", c);
}
