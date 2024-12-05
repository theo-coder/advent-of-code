fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part1(input);
    dbg!(output);
}

fn solve_part1(input: &str) -> i64 {
    let mut result = 0;
    for report in input.lines() {
        let mut levels: Vec<i32> = report.split(" ").map(|s| s.parse().unwrap()).collect();

        if levels[0] < levels[1] {
            levels.reverse();
        }

        for i in 1..levels.len() {
            if levels[i - 1] <= levels[i] {
                break;
            }

            if (levels[i - 1] - levels[i]).abs() > 3 {
                break;
            }

            if i == levels.len() - 1 {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn it_works() {
        let result = solve_part1(INPUT);

        assert_eq!(result, 2);
    }
}
