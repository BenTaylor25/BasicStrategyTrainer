use rand::Rng;

pub fn get_rand_card() -> char {
    let cards = ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..13);

    cards[i]
}
