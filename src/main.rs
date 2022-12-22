mod card;
mod display;
mod basic_strategy;

use std::io::{self, stdin, Write};
use colored::Colorize;

fn menu_input(input_string: &mut String) {
    println!("Hit (h), Stand (s), Double Down (d), Split (2), Split iff DAS (5) or Quit (q)");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input_string.clear();
        stdin().read_line(input_string)
            .ok()
            .expect("Failed to read input");

        match input_string.trim() {
            "h" | "s" | "d" | "2" | "q" | "5" => break,
            _ => continue
        }
    }
}

fn main() {
    let mut keep_playing = true;
    while keep_playing {
        let cards = card::get_rand_deck(3);
        let player_total = basic_strategy::calc_player_total(&cards);
        let player_blackjack = player_total.soft && player_total.total == 11;

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
                let mut correct_char = String::new();
                let _ = basic_strategy::check_move(&cards, '!', &mut correct_char);
                println!("{}", correct_char);
            } else {
                let mut correct_char = String::new();
                let guessed_correct = basic_strategy::check_move(&cards, input_chars[0], &mut correct_char);
                if guessed_correct {
                    println!("{}", format!("Correct!").green());
                } else {
                    println!("{}", format!("Incorrect!").red());
                    println!("{}", correct_char);
                }
                println!();
            }
        }
        println!();
    }
}
