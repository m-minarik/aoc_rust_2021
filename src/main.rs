use std::fs;

// mod d16;
// mod d17;
mod d18;

fn main() {
    let input: String = fs::read_to_string("data/d18.test").unwrap();
    d18::part_1(&input);
    d18::part_2(&input);
}
