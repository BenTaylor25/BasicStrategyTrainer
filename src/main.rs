mod card;
mod display;
mod basic_strategy;

use std::io::{self, stdin, Write};

fn menu_input(input_string: &mut String) {
    println!("Hit (h), Stand (s), Double Down (d), or Split (2)");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input_string.clear();
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

    let mut input_string = String::new();
    menu_input(&mut input_string);

    let input_chars: Vec<char> = input_string.chars().collect();

    let _correct_guess = basic_strategy::check_move(&cards, input_chars[0]);
}
