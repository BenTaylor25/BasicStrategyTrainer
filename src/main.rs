mod card;
mod display;
mod basic_strategy;

use std::io::{self, stdin, Write};

fn menu_input(input_string: &mut String) {
    println!("Hit (h), Stand (s), Double Down (d), Split (2), or Quit (q)");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input_string.clear();
        stdin().read_line(input_string)
            .ok()
            .expect("Failed to read input");

        match input_string.trim() {
            "h" | "s" | "d" | "2" | "q" => break,
            _ => continue
        }
    }
}

fn main() {
    let mut keep_playing = true;
    while keep_playing {
        let cards = card::get_rand_deck(3);
        let player_blackjack = 
            card::get_card_value(cards[1]) +
            card::get_card_value(cards[2]) == 21;

        display::print_cards(&cards);

        if player_blackjack {
            println!("Blackjack!");
            println!();
        } else {
            let mut input_string = String::new();
            menu_input(&mut input_string);
            let input_chars: Vec<char> = input_string.chars().collect();

            if input_chars[0] == 'q' {
                keep_playing = false;
            } else {
                let correct_guess = basic_strategy::check_move(&cards, input_chars[0]);
                if correct_guess {
                    println!("Correct!");
                } else {
                    println!("Incorrect!");
                }
                println!();
            }
        }
        println!();
    }
}
