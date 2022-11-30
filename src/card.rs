use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub enum Card {
    ACE = 'A',
    ONE = '1',
    TWO = '2',
    THREE = '3',
    FOUR = '4',
    FIVE = '5',
    SIX = '6',
    SEVEN = '7',
    EIGHT = '8',
    NINE = '9',
    TEN = 'T',
    JACK = 'J',
    QUEEN = 'Q',
    KING = 'K'
}

impl Distribution<Card> for Standard {
    fn sample<R : Rng + ?Sized>(&self, rng: &mut R) -> Card {
        match rng.gen_range(0..=13) {
            0 => Card::ACE,
            1 => Card::ONE,
            2 => Card::TWO,
            3 => Card::THREE,
            4 => Card::FOUR,
            5 => Card::FIVE,
            6 => Card::SIX,
            7 => Card::SEVEN,
            8 => Card::EIGHT,
            9 => Card::NINE,
            10 => Card::TEN,
            11 => Card::JACK,
            12 => Card::QUEEN,
            _ => Card::KING,
        }
    }
}

fn get_rand_card() -> Card {
    let card: Card = rand::random();
    card
}
