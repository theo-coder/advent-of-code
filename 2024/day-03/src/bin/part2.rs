use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part2(input);
    dbg!(output);
}

fn solve_part2(input: &str) -> i64 {
    let mut result: i64 = 0;
    let (mut to_parse, mut rest) = input.split_once("don't()").unwrap_or((input, ""));

    while !rest.is_empty() || !to_parse.is_empty() {
        let re = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();

        for mul in re.captures_iter(to_parse) {
            result += mul[1].parse::<i64>().unwrap() * mul[2].parse::<i64>().unwrap();
        }

        if let Some(next) = rest.split_once("do()").map(|(_, to_parse)| to_parse) {
            to_parse = next
                .split_once("don't()")
                .map(|(to_parse, _)| to_parse)
                .unwrap_or(next);
            rest = next
                .split_once("don't()")
                .map(|(_, rest)| rest)
                .unwrap_or("");
        } else {
            to_parse = "";
            rest = "";
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = solve_part2(input);

        assert_eq!(result, 48);
    }

    #[test]
    fn t2() {
        let input = "mul(10,10)do()mul(20,20)don't()mul(5,5)";
        let result = solve_part2(input);

        assert_eq!(result, 500);
    }

    #[test]
    fn t3() {
        let input = "don't()mul(5,10)do()mul(20,2)don't()mul(3,7)do()mul(8,8)don't()mul(4,4)";
        let result = solve_part2(input);

        assert_eq!(result, 104);
    }

    #[test]
    fn t4() {
        let input = "mul(10,10)don't()mul(5,5)do()mul(3,3)don't()do()don't()mul(1,1)do()mul(6,6)";
        let result = solve_part2(input);

        assert_eq!(result, 145);
    }

    #[test]
    fn t5() {
        let input = "do()do()mul(2,2)don't()don't()don't()mul(3,3)do()do()mul(4,4)";
        let result = solve_part2(input);

        assert_eq!(result, 20);
    }
}
