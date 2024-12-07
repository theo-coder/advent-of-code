fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part1(input);
    dbg!(output);
}

fn solve_part1(input: &str) -> i64 {
    let mut result = 0;

    let operations: Vec<_> = input
        .lines()
        .map(|l| {
            let (res, nbs) = l.split_once(": ").unwrap();
            (
                res.parse::<i64>().unwrap(),
                nbs.split(" ")
                    .map(|nb| nb.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    'outer: for operation in &operations {
        let obj = operation.0;

        let mut possible_operations = Vec::new();
        let nb_calc = 2_i64.pow((operation.1.len() - 1) as u32);

        for i in 0..nb_calc {
            let mut calc_tpl = format!("{:b}", i);
            while calc_tpl.len() < format!("{:b}", nb_calc - 1).len() {
                calc_tpl.insert(0, '0');
            }
            possible_operations.push(calc_tpl);
        }

        for op in possible_operations {
            let res = calc_from_binary(&operation.1, &op);
            if res == obj {
                result += res;
                continue 'outer;
            }
        }
    }

    result
}

fn calc_from_binary(nbs: &[i64], binary_ops: &str) -> i64 {
    let mut result = nbs[0];

    for (nb_idx, nb) in nbs.iter().enumerate().skip(1) {
        match binary_ops.chars().nth(nb_idx - 1).unwrap() {
            '0' => result += nb,
            '1' => result *= nb,
            _ => unreachable!(),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn it_works() {
        let result = solve_part1(INPUT);

        assert_eq!(result, 3749);
    }
}
