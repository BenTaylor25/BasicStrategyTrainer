use rand::Rng;

pub fn get_rand_card() -> char {
    let cards = ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..13);

    cards[i]
}

pub fn get_rand_deck(size: usize) -> Vec<char> {
    (0..size).map(|_| get_rand_card()).collect()
}

pub fn get_card_value(card: char) -> i8 {
    let cards = ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];

    let mut val: i8 = 0;
    for c in 0..cards.len() {
        if cards[c] == card {
            val = match i8::try_from(c).ok() {
                Some(i8c) => { i8c + 1 },
                _ => { 0 }
            }
        }
    }
    if val > 10 {
        val = 10;
    }

    assert!(val != 0);

    val
}