fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part2(input);
    dbg!(output);
}

fn solve_part2(input: &str) -> i64 {
    let mut result = 0;
    for report in input.lines() {
        let levels: Vec<i32> = report.split(" ").map(|s| s.parse().unwrap()).collect();

        match process_report(levels.clone()) {
            Ok(()) => result += 1,
            Err(idx) => {
                let mut try_one = levels.clone();
                try_one.remove(idx);
                if process_report(try_one.clone()).is_ok() {
                    result += 1
                } else {
                    let mut try_two = levels.clone();
                    if idx >= 1 {
                        try_two.remove(idx - 1);
                    }

                    if process_report(try_two.clone()).is_ok() {
                        result += 1
                    } else {
                        let mut try_three = levels.clone();
                        if idx <= try_three.len() - 2 {
                            try_three.remove(idx + 1);
                        }

                        if process_report(try_three.clone()).is_ok() {
                            result += 1
                        }
                    }
                }
            }
        }
    }
    result
}

fn process_report(levels: Vec<i32>) -> Result<(), usize> {
    let mut reversed = false;
    let mut levels = levels.clone();
    if determine_reversed(&levels) {
        reversed = true;
        levels.reverse();
    }

    for i in 1..levels.len() {
        if levels[i - 1] <= levels[i] {
            return Err(if reversed { levels.len() - i } else { i - 1 });
        }

        if (levels[i - 1] - levels[i]).abs() > 3 {
            return Err(if reversed { levels.len() - i } else { i - 1 });
        }
    }
    Ok(())
}

fn determine_reversed(levels: &[i32]) -> bool {
    levels.windows(2).filter(|pair| pair[0] < pair[1]).count() > levels.len() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7";

    #[test]
    fn part2() {
        let result = solve_part2(INPUT);

        assert_eq!(result, 8);
    }
}
