use std::fs;

mod solutions;

pub const A_TOKEN: isize = 3;
pub const B_TOKEN: isize = 1;

#[derive(Debug)]
pub struct Game {
    pub a: (isize, isize),
    pub b: (isize, isize),
    pub prize: (isize, isize),
}

pub struct Puzzle {
    pub games: Vec<Game>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).unwrap();
        Self::from_str(&input)
    }
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let mut games = vec![];

        for parsed_game in input.split("\n\n") {
            let mut inputs = parsed_game.lines();

            games.push(Game {
                a: Self::parse_button(inputs.next().unwrap()),
                b: Self::parse_button(inputs.next().unwrap()),
                prize: Self::parse_prize(inputs.next().unwrap()),
            });
        }

        Self { games }
    }

    fn parse_button(button: &str) -> (isize, isize) {
        let button = button[12..button.len()].replace(" Y+", "");
        let button = button.split_once(",").unwrap();
        (button.0.parse().unwrap(), button.1.parse().unwrap())
    }

    fn parse_prize(prize: &str) -> (isize, isize) {
        let prize = prize[9..prize.len()].replace(" Y=", "");
        let prize = prize.split_once(",").unwrap();
        (prize.0.parse().unwrap(), prize.1.parse().unwrap())
    }
}

fn main() {
    let puzzle = Puzzle::from("./src/input.txt");

    dbg!(puzzle.part_1());
    dbg!(puzzle.part_2());
}
