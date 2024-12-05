fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part1(input);
    dbg!(output);
}

fn solve_part1(input: &str) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn it_works() {
        let result = solve_part1(INPUT);

        assert_eq!(result, 0);
    }
}
