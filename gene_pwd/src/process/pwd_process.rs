use char_base::{LOWER_CASE, NUMBER, SYMBOL, UPPER_CASE};
use rand::seq::{IndexedRandom, SliceRandom};

pub fn generate_password(length: u8, uppercase: bool, number: bool, symbol: bool) -> String {
    let mut rng = rand::rng();
    let mut chars = Vec::new();
    let mut return_val = Vec::new();
    if uppercase {
        chars.extend_from_slice(UPPER_CASE);
        chars.extend_from_slice(LOWER_CASE);
    } else {
        chars.extend_from_slice(LOWER_CASE);
    }

    if number {
        chars.extend_from_slice(NUMBER);
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
    }

    for _ in 0..length {
        let c = chars.choose(&mut rng).expect("chars can not be empty!");
        return_val.push(*c);
    }

    return_val.shuffle(&mut rng);

    String::from_utf8(return_val).unwrap()
}
