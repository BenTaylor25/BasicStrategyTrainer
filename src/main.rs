mod card;
mod display;

fn main() {
    let cards = card::get_rand_deck(3);

    display::print_cards(&cards);
}
