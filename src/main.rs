mod card;
use card::{get_rand_card};

fn main() {
    let c = get_rand_card();

    println!("{}", c);
}
