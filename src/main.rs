mod card;
use card::{get_rand_card};
mod display;
use display::display;

fn main() {
    let cards = [
        get_rand_card(),
        get_rand_card(),
        get_rand_card(),
    ];

    display(&cards);
}
