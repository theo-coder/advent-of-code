use crate::{Puzzle, A_TOKEN, B_TOKEN};

impl Puzzle {
    pub fn part_2(&self) -> isize {
        let mut total_cost = 0;
        for game in &self.games {
            let prize = (game.prize.0 + 10000000000000, game.prize.1 + 10000000000000);

            let b = (game.a.0 * prize.1 - game.a.1 * prize.0)
                / (game.a.0 * game.b.1 - game.a.1 * game.b.0);
            let a = (prize.0 - game.b.0 * b) / game.a.0;

            total_cost += if game.a.0 * a + game.b.0 * b == prize.0
                && game.a.1 * a + game.b.1 * b == prize.1
            {
                a * A_TOKEN + b * B_TOKEN
            } else {
                0
            }
        }
        total_cost
    }
}
