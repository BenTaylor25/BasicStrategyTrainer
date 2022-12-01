
pub fn display(cards: &[char]) {
    let player_card_count = cards.len() - 1;

    if player_card_count < 2 {
        panic!("Tried to display less than 3 cards");
    }

    println!("+---+");
    println!("| {} |", cards[0]);
    println!("+---+");

    for _ in 1..=player_card_count {
        print!("+---+ ");
    }
    println!();

    for i in 1..=player_card_count {
        print!("| {} | ", cards[i]);
    }
    println!();

    for _ in 1..=player_card_count {
        print!("+---+ ")
    }
    println!();
}
