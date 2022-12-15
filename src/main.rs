mod card;
mod display;

use std::io::{self, stdin, Write};

fn menu_input(input_string: &mut String) {
    println!("Hit (h), Stand (s), Double Down (d), or Split (2)");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        stdin().read_line(input_string)
            .ok()
            .expect("Failed to read input");

        match input_string.trim() {
            "h" | "s" | "d" | "2" => break,
            _ => continue
        }
    }
}

fn main() {
    let cards = card::get_rand_deck(3);

    display::print_cards(&cards);

    let mut menu_option = String::new();
    menu_input(&mut menu_option);

    println!("{}", menu_option);
}
