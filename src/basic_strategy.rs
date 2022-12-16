use crate::card;

struct Total {
    total: i8,
    soft: bool
}

fn calc_player_total(cards: Vec<char>) -> Total {
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

pub fn check_move(cards: Vec<char>, choice: char) -> bool {
    let mut dealer_total = card::get_card_value(cards[0]);
    if dealer_total == 1 {
        dealer_total = 10;   // in Basic Strategy, dealer Ace is almost always the same as dealer Ten
    }
    let player_total = calc_player_total(cards);

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
