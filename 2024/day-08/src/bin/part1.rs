use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part1(input);
    dbg!(output);
}

fn solve_part1(input: &str) -> usize {
    let mut map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    map.iter().enumerate().for_each(|(l_idx, l)| {
        l.iter()
            .enumerate()
            .filter(|(_, c)| **c != '.' && **c != '#')
            .for_each(|(c_idx, c)| {
                let antenna = antennas.entry(*c).or_default();
                antenna.push((c_idx as i64, l_idx as i64));
            })
    });

    for (_, positions) in antennas {
        positions.windows(2).for_each(|pos| {
            let [pos_1, pos_2] = pos else { unreachable!() };
            let (new_pos_1, new_pos_2) = check_new_pos(pos_1, pos_2);

            if new_pos_1.0 >= 0
                && new_pos_1.0 < map[0].len() as i64
                && new_pos_1.1 >= 0
                && new_pos_1.1 < map[0].len() as i64
            {
                map[new_pos_1.1 as usize][new_pos_1.0 as usize] = '#';
            }

            if new_pos_2.0 >= 0
                && new_pos_2.0 < map[0].len() as i64
                && new_pos_2.1 >= 0
                && new_pos_2.1 < map[0].len() as i64
            {
                map[new_pos_2.1 as usize][new_pos_2.0 as usize] = '#';
            }
        });

        for i in 0..positions.len() {
            for j in i + 2..positions.len() {
                let [pos_1, pos_2] = [positions[i], positions[j]];
                let (new_pos_1, new_pos_2) = check_new_pos(&pos_1, &pos_2);

                if new_pos_1.0 >= 0
                    && new_pos_1.0 < map[0].len() as i64
                    && new_pos_1.1 >= 0
                    && new_pos_1.1 < map[0].len() as i64
                {
                    map[new_pos_1.1 as usize][new_pos_1.0 as usize] = '#';
                }

                if new_pos_2.0 >= 0
                    && new_pos_2.0 < map[0].len() as i64
                    && new_pos_2.1 >= 0
                    && new_pos_2.1 < map[0].len() as i64
                {
                    map[new_pos_2.1 as usize][new_pos_2.0 as usize] = '#';
                }
            }
        }
    }

    map.iter().flatten().filter(|c| **c == '#').count()
}

fn check_new_pos(pos_1: &(i64, i64), pos_2: &(i64, i64)) -> ((i64, i64), (i64, i64)) {
    let new_diff_x = pos_2.0 - pos_1.0;
    let new_diff_y = pos_2.1 - pos_1.1;

    let new_pos_1 = (pos_1.0 - new_diff_x, pos_1.1 - new_diff_y);
    let new_pos_2 = (pos_2.0 + new_diff_x, pos_2.1 + new_diff_y);

    (new_pos_1, new_pos_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn it_works() {
        let result = solve_part1(INPUT);

        assert_eq!(result, 14);
    }
}
