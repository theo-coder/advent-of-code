use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part2(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Facing {
    N,
    S,
    E,
    W,
}

fn solve_part2(input: &str) -> i64 {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut y = map.iter().position(|l| l.contains(&'^')).unwrap();
    let mut x = map[y].iter().position(|c| c == &'^').unwrap();

    let base_pos = (x, y);

    let mut facing = Facing::N;

    let _ = do_loop(&mut facing, &mut x, &mut y, &mut map);

    y = base_pos.1;
    x = base_pos.0;
    facing = Facing::N;

    let mut result = 0;

    for (line_idx, line) in map.iter().enumerate() {
        for (col_idx, char) in line.iter().enumerate() {
            if *char == 'X' {
                let mut map = map.clone();
                map[line_idx][col_idx] = '#';

                if do_loop(&mut facing, &mut x, &mut y, &mut map) {
                    result += 1
                }

                facing = Facing::N;
                y = base_pos.1;
                x = base_pos.0;
            }
        }
    }

    result
}

fn do_loop(facing: &mut Facing, x: &mut usize, y: &mut usize, map: &mut [Vec<char>]) -> bool {
    let mut history: HashSet<(Facing, usize, usize)> = HashSet::new();
    loop {
        if history.contains(&(facing.clone(), *x, *y)) {
            return true;
        }

        history.insert((facing.clone(), *x, *y));

        if !do_move(facing, x, y, map) {
            break;
        }
    }
    false
}

fn do_move(facing: &mut Facing, x: &mut usize, y: &mut usize, map: &mut [Vec<char>]) -> bool {
    map[*y][*x] = 'X';

    match facing {
        Facing::N => {
            if *y == 0 {
                return false;
            }

            if map[*y - 1][*x] == '#' {
                *facing = Facing::E;
                return true;
            }

            *y -= 1;
        }
        Facing::S => {
            if *y == map.len() - 1 {
                return false;
            }

            if map[*y + 1][*x] == '#' {
                *facing = Facing::W;
                return true;
            }

            *y += 1;
        }
        Facing::E => {
            if *x == map[0].len() - 1 {
                return false;
            }

            if map[*y][*x + 1] == '#' {
                *facing = Facing::S;
                return true;
            }

            *x += 1;
        }
        Facing::W => {
            if *x == 0 {
                return false;
            }

            if map[*y][*x - 1] == '#' {
                *facing = Facing::N;
                return true;
            }

            *x -= 1;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn it_works() {
        let result = solve_part2(INPUT);

        assert_eq!(result, 6);
    }
}
