fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part1(input);
    dbg!(output);
}

fn solve_part1(input: &str) -> i64 {
    let mut result = 0;

    let (rules, updates) = input
        .split_once("\n\n")
        .map(|(rules, updates)| {
            (
                rules
                    .lines()
                    .map(|l| l.split_once("|").unwrap())
                    .collect::<Vec<_>>(),
                updates
                    .lines()
                    .map(|l| l.split(",").collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap();

    for update in updates {
        let middle = update[update.len() / 2];
        let mut update_ok = true;

        for rule in &rules {
            if update.contains(&rule.0) && update.contains(&rule.1) {
                // println!("{}|{} : {:?}", rule.0, rule.1, update);
                let pos0 = update.iter().position(|nb| nb == &rule.0).unwrap();
                let pos1 = update.iter().position(|nb| nb == &rule.1).unwrap();

                if pos0 > pos1 {
                    update_ok = false;
                }
            }
        }

        if update_ok {
            result += middle.parse::<i64>().unwrap();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn it_works() {
        let result = solve_part1(INPUT);

        assert_eq!(result, 143);
    }
}
