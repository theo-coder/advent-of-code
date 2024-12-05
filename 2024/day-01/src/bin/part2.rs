fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part2(input);
    dbg!(output);
}

fn solve_part2(input: &str) -> i64 {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut split = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());

        left.push(split.next().unwrap());
        right.push(split.next().unwrap());
    }

    let result = left
        .iter()
        .map(|l| l * right.iter().filter(|r| &l == r).count() as i32)
        .sum::<i32>();

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
3   3";

    #[test]
    fn it_works() {
        let result = solve_part2(INPUT);

        assert_eq!(result, 31);
    }
}
