use std::fs;

mod solutions;

pub struct Puzzle {}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).unwrap();
        Self::from_str(&input)
    }
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        Self {}
    }
}

fn main() {
    let puzzle = Puzzle::from("./src/input.txt");

    dbg!(puzzle.part_1());
    dbg!(puzzle.part_2());
}
