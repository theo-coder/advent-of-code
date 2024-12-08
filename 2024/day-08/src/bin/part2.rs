use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part2(input);
    dbg!(output);
}

fn solve_part2(input: &str) -> usize {
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
            let diff_x = pos_2.0 - pos_1.0;
            let diff_y = pos_2.1 - pos_1.1;

            let mut new_pos_1 =
                check_new_pos(pos_1, &diff_x, &diff_y, map[0].len(), map.len(), true);

            while let Some(new_pos) = new_pos_1 {
                map[new_pos.1 as usize][new_pos.0 as usize] = '#';
                new_pos_1 =
                    check_new_pos(&new_pos, &diff_x, &diff_y, map[0].len(), map.len(), true);
            }

            let mut new_pos_2 =
                check_new_pos(pos_2, &diff_x, &diff_y, map[0].len(), map.len(), false);

            while let Some(new_pos) = new_pos_2 {
                map[new_pos.1 as usize][new_pos.0 as usize] = '#';
                new_pos_2 =
                    check_new_pos(&new_pos, &diff_x, &diff_y, map[0].len(), map.len(), false);
            }
        });

        for i in 0..positions.len() {
            for j in i + 2..positions.len() {
                let [pos_1, pos_2] = [positions[i], positions[j]];

                let diff_x = pos_2.0 - pos_1.0;
                let diff_y = pos_2.1 - pos_1.1;

                let mut new_pos_1 =
                    check_new_pos(&pos_1, &diff_x, &diff_y, map[0].len(), map.len(), true);

                while let Some(new_pos) = new_pos_1 {
                    map[new_pos.1 as usize][new_pos.0 as usize] = '#';
                    new_pos_1 =
                        check_new_pos(&new_pos, &diff_x, &diff_y, map[0].len(), map.len(), true);
                }

                let mut new_pos_2 =
                    check_new_pos(&pos_2, &diff_x, &diff_y, map[0].len(), map.len(), false);

                while let Some(new_pos) = new_pos_2 {
                    map[new_pos.1 as usize][new_pos.0 as usize] = '#';
                    new_pos_2 =
                        check_new_pos(&new_pos, &diff_x, &diff_y, map[0].len(), map.len(), false);
                }
            }
        }
    }

    map.iter().flatten().filter(|c| **c != '.').count()
}

fn check_new_pos(
    pos: &(i64, i64),
    diff_x: &i64,
    diff_y: &i64,
    max_x: usize,
    max_y: usize,
    before: bool,
) -> Option<(i64, i64)> {
    let new_pos = if before {
        (pos.0 - diff_x, pos.1 - diff_y)
    } else {
        (pos.0 + diff_x, pos.1 + diff_y)
    };

    if new_pos.0 < 0 || new_pos.0 >= max_x as i64 || new_pos.1 < 0 || new_pos.1 >= max_y as i64 {
        return None;
    }

    Some(new_pos)
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
        let result = solve_part2(INPUT);

        assert_eq!(result, 34);
    }
}
