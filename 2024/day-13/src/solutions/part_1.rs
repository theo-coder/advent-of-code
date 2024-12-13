use crate::{Puzzle, A_TOKEN, B_TOKEN};

impl Puzzle {
    pub fn part_1(&self) -> isize {
        let mut total_cost = 0;
        for game in &self.games {
            let mut minimum_cost = None;

            for a in 1..100 {
                for b in 1..100 {
                    if game.a.0 * a + game.b.0 * b == game.prize.0
                        && game.a.1 * a + game.b.1 * b == game.prize.1
                    {
                        let cost = a * A_TOKEN + b * B_TOKEN;

                        if let Some(old_minimum) = minimum_cost {
                            if cost < old_minimum {
                                minimum_cost = Some(cost);
                            }
                        } else {
                            minimum_cost = Some(cost);
                        }
                    }
                }
            }

            if let Some(cost) = minimum_cost {
                total_cost += cost;
            }
        }
        total_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        let puzzle = Puzzle::from_str(input);

        assert_eq!(puzzle.part_1(), 480);
    }
}
