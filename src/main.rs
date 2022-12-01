mod card;
use card::{get_rand_card};
mod display;
use display::display;

fn main() {
    let c = get_rand_card();

    println!("{}", c);


    let cards = ['k', 'j', 'q'];
    display(&cards);
}
