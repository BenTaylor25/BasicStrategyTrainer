use crate::card;

struct Total {
    total: i8,
    soft: bool
}

fn calc_player_total(cards: &Vec<char>) -> Total {
    let mut has_ace = false;
    let mut total = 0i8;

    for i in 1..cards.len() {
        if cards[i] == 'A' {
            has_ace = true;
        }

        total += card::get_card_value(cards[i]);
    }

    Total { total, soft: has_ace && total <= 11 }
}

fn player_can_split(cards: &Vec<char>) -> bool {
    cards[1] == cards[2] && cards.len() == 3
}

pub fn check_move(cards: &Vec<char>, choice: char) -> bool {
    let mut dealer_total = card::get_card_value(cards[0]);
    if dealer_total == 1 {
        dealer_total = 10;   // in Basic Strategy, dealer Ace is almost always the same as dealer Ten
    }
    let player_total = calc_player_total(&cards);
    let can_split = player_can_split(&cards);

    if can_split {
        // Double after Split rule
        const DAS: bool = true;

        if cards[1] == 'A' || cards[1] == '8' {
            return choice == '2';
        }

        if cards[1] == '9' && dealer_total < 10 && cards[0] != '7' {
            return choice == '2';
        }

        if cards[1] == '7' && dealer_total < 8 {
            return choice == '2';
        }

        if cards[1] == '6' {
            if dealer_total < 7 && !(cards[0] == '2' && !DAS) {
                return choice == '2';
            }
        }

        if cards[1] == '4' && (cards[0] == '5' || cards[0] == '6') && DAS {
            return choice == '2';
        }

        if (cards[1] == '2' || cards[1] == '3') && dealer_total < 8 {
            if dealer_total > 3 || DAS {
                return choice == '2';
            }
        }
    }

    if player_total.soft {
        return false; //
    }

    // hard totals
    if player_total.total >= 17 {
        return choice == 's';
    }

    if player_total.total <= 8 {
        return choice == 'h';
    }

    if player_total.total >= 13 {
        if dealer_total >= 7 {
            return choice == 'h';
        }
        return choice == 's';
    }

    if player_total.total == 12 {
        if dealer_total >= 4 && dealer_total <= 6 {
            return choice == 's';
        }
        return choice == 'h';
    }

    if player_total.total == 11 {
        return choice == 'd';
    }

    if player_total.total == 10 {
        if dealer_total >= 10 {
            return choice == 'h';
        }
        return choice == 'd';
    }

    // player total == 9
    if dealer_total >= 3 && dealer_total <= 6 {
        return choice == 'd';
    }
    return choice == 'h';


}
