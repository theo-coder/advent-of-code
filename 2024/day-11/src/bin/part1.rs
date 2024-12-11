fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part1(input.trim_end());
    dbg!(output);
}

const BLINKS: u8 = 25;

fn solve_part1(input: &str) -> usize {
    let mut stones = input
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();

    for _ in 0..BLINKS {
        let mut new_stones = Vec::new();
        for stone in &stones {
            if *stone == 0 {
                new_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let stone = stone.to_string();
                let (left, right) = stone.split_at(stone.to_string().len() / 2);
                new_stones.push(left.parse().unwrap());
                new_stones.push(right.parse().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones
    }

    stones.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn it_works() {
        let result = solve_part1(INPUT);

        assert_eq!(result, 55312);
    }
}
