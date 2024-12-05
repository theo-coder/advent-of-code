use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part1(input);
    dbg!(output);
}

fn solve_part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();

    let result: i32 = re
        .captures_iter(input)
        .map(|mul| mul[1].parse::<i32>().unwrap() * mul[2].parse::<i32>().unwrap())
        .sum();

    result.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn it_works() {
        let result = solve_part1(INPUT);

        assert_eq!(result, 161);
    }
}
