use crate::Puzzle;

impl Puzzle {
    pub fn part_2(&self) -> isize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "";

        let puzzle = Puzzle::from_str(input);

        assert_eq!(puzzle.part_2(), 0);
    }
}
