use crate::card;

pub struct Total {
    pub total: i8,
    pub soft: bool
}

pub fn calc_player_total(cards: &Vec<char>) -> Total {
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

fn player_can_double_down(cards: &Vec<char>) -> bool {
    cards.len() == 3
}

pub fn check_move(cards: &Vec<char>, choice: char, correct_char: &mut String) -> bool {
    let mut dealer_total = card::get_card_value(cards[0]);
    if dealer_total == 1 {
        dealer_total = 10;   // in Basic Strategy, dealer Ace is almost always the same as dealer Ten
    }
    let player_total = calc_player_total(&cards);
    let can_split = player_can_split(&cards);
    let can_double = player_can_double_down(&cards);

    if can_split {
        // Double after Split rule
        const DAS: bool = true;

        if cards[1] == 'A' || cards[1] == '8' {
            correct_char.push('2');
            return choice == '2';
        }

        if cards[1] == '9' && dealer_total < 10 && cards[0] != '7' {
            correct_char.push('2');
            return choice == '2';
        }

        if cards[1] == '7' && dealer_total < 8 {
            correct_char.push('2');
            return choice == '2';
        }

        if cards[1] == '6' {
            if dealer_total < 7 && !(cards[0] == '2' && !DAS) {
                correct_char.push('2');
                return choice == '2';
            }
        }

        if cards[1] == '4' && (cards[0] == '5' || cards[0] == '6') && DAS {
            correct_char.push('2');
            return choice == '2';
        }

        if (cards[1] == '2' || cards[1] == '3') && dealer_total < 8 {
            if dealer_total > 3 || DAS {
                correct_char.push('2');
                return choice == '2';
            }
        }
    }

    if player_total.soft {
        // player total = player_total.total or player_total.total + 10

        // A + 9
        if player_total.total == 10 {
            correct_char.push('s');
            return choice == 's';
        }

        // A + 8
        if player_total.total == 9 {
            if dealer_total == 6 && can_double {
                correct_char.push('d');
                return choice == 'd';
            }
            correct_char.push('s');
            return choice == 's';
        }

        // A + 7
        if player_total.total == 8 {
            if dealer_total > 8 {
            correct_char.push('h');
                return choice == 'h';
            }
            if dealer_total < 7 && can_double {
            correct_char.push('d');
                return choice == 'd';
            }
            correct_char.push('s');
            return choice == 's';
        }

        // A + 6
        if player_total.total == 7 {
            if dealer_total > 2 && dealer_total < 7 && can_double {
            correct_char.push('d');
                return choice == 'd';
            }
            correct_char.push('h');
            return choice == 'h';
        }

        // A + 5 or A + 4
        if player_total.total > 4 {
            if dealer_total > 3 && dealer_total < 7 {
            correct_char.push('d');
                return choice == 'd';
            }
            correct_char.push('h');
            return choice == 'h';
        }

        // A + 3 or A + 2
        if dealer_total > 4 && dealer_total < 7 {
            correct_char.push('d');
            return choice == 'd';
        }
            correct_char.push('h');
        return choice == 'h';
    }

    // hard totals
    if player_total.total >= 17 {
            correct_char.push('s');
        return choice == 's';
    }

    if player_total.total <= 8 {
            correct_char.push('h');
        return choice == 'h';
    }

    if player_total.total >= 13 {
        if dealer_total >= 7 {
            correct_char.push('h');
            return choice == 'h';
        }
            correct_char.push('s');
        return choice == 's';
    }

    if player_total.total == 12 {
        if dealer_total >= 4 && dealer_total <= 6 {
            correct_char.push('s');
            return choice == 's';
        }
            correct_char.push('h');
        return choice == 'h';
    }

    if player_total.total == 11 {
        if can_double {
            correct_char.push('d');
            return choice == 'd';
        }
        correct_char.push('h');
        return choice == 'h';
        
    }

    if player_total.total == 10 {
        if dealer_total >= 10 {
        correct_char.push('h');
            return choice == 'h';
        }
        if can_double {
        correct_char.push('d');
            return choice == 'd';
        }
        correct_char.push('h');
        return choice == 'h';
    }

    // player total == 9
    if dealer_total >= 3 && dealer_total <= 6 && can_double {
        correct_char.push('d');
        return choice == 'd';
    }
        correct_char.push('h');
    return choice == 'h';


}
