fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part1(input);
    dbg!(output);
}

fn solve_part1(input: &str) -> i64 {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut split = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());

        left.push(split.next().unwrap());
        right.push(split.next().unwrap());
    }

    left.sort();
    right.sort();

    let result: i32 = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    result.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn it_works() {
        let result = solve_part1(INPUT);

        assert_eq!(result, 11);
    }
}
