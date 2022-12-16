mod card;

struct Total {
    total: i8,
    soft: bool
}

fn calc_player_total(cards: Vec<char>) -> Total {
    let has_ace = false;
    let total = 0;

    for i in 1..cards.len() {
        if cards[i] == 'A' {
            has_ace = true;
        }


    }
}

pub fn check_move(cards: Vec<char>, choice: char) -> bool {
    let player_total = calc_player_total(cards);
}
