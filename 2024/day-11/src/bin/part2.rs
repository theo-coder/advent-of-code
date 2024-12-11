use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part2(input.trim_end());
    dbg!(output);
}

const BLINKS: u8 = 75;

fn solve_part2(input: &str) -> usize {
    let mut stones = input
        .split(" ")
        .map(|n| (n.parse().unwrap(), 1))
        .collect::<HashMap<usize, usize>>();

    for _ in 0..BLINKS {
        let mut new_stones = HashMap::new();
        for (stone, count) in stones {
            let (s1, s2) = match stone {
                0 => (1, None),
                _ if stone.to_string().len() % 2 == 0 => {
                    let stone = stone.to_string();
                    let (left, right) = stone.split_at(stone.to_string().len() / 2);
                    (left.parse().unwrap(), Some(right.parse().unwrap()))
                }
                nb => (nb * 2024, None),
            };

            new_stones
                .entry(s1)
                .and_modify(|new_count| *new_count += count)
                .or_insert(count);

            if let Some(s2) = s2 {
                new_stones
                    .entry(s2)
                    .and_modify(|new_count| *new_count += count)
                    .or_insert(count);
            }
        }
        stones = new_stones;
    }

    stones.into_values().reduce(|acc, c| acc + c).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let a = 1 == 1;
        assert!(a)
    }
}
