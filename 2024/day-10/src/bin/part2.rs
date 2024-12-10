use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part2(input);
    dbg!(output);
}

fn solve_part2(input: &str) -> usize {
    let mut map = Vec::new();
    let mut potential_heads = Vec::new();

    for (l_idx, line) in input.lines().enumerate() {
        map.push(Vec::new());
        for (c_idx, nb) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            map[l_idx].push(nb);
            if nb == 0 {
                potential_heads.push(((c_idx, l_idx), (c_idx, l_idx), 1));
            }
        }
    }

    let mut trails_from_head = HashMap::new();

    potential_heads.iter().for_each(|h| {
        trails_from_head.insert(h.0, 0);
    });

    while let Some((current, initial, level)) = potential_heads.pop() {
        if level > 9 {
            *trails_from_head.get_mut(&initial).unwrap() += 1;
            continue;
        }

        let possible_bounds = check_bounds(current, level, &map);

        for possible_bound in possible_bounds {
            potential_heads.push((possible_bound, initial, level + 1));
        }
    }

    trails_from_head.iter().map(|t| t.1).sum()
}

fn check_bounds(head: (usize, usize), i: usize, map: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut possible_bounds = Vec::new();

    if head.1 > 0 && map[head.1 - 1][head.0] == i as u32 {
        possible_bounds.push((head.0, head.1 - 1));
    }

    if head.1 < map.len() - 1 && map[head.1 + 1][head.0] == i as u32 {
        possible_bounds.push((head.0, head.1 + 1));
    }

    if head.0 > 0 && map[head.1][head.0 - 1] == i as u32 {
        possible_bounds.push((head.0 - 1, head.1));
    }

    if head.0 < map[0].len() - 1 && map[head.1][head.0 + 1] == i as u32 {
        possible_bounds.push((head.0 + 1, head.1));
    }

    possible_bounds
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn it_works() {
        let result = solve_part2(INPUT);

        assert_eq!(result, 81);
    }
}
