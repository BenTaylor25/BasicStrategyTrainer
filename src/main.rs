mod card;
mod display;

fn main() {
    let cards = [
        card::get_rand_card(),
        card::get_rand_card(),
        card::get_rand_card(),
    ];

    display::print_cards(&cards);
}
