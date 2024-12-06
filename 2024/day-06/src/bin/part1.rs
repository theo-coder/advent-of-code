fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part1(input);
    dbg!(output);
}

#[derive(Debug)]
enum Facing {
    N,
    S,
    E,
    W,
}

fn solve_part1(input: &str) -> i64 {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut y = map.iter().position(|l| l.contains(&'^')).unwrap();
    let mut x = map[y].iter().position(|c| c == &'^').unwrap();

    let mut facing = Facing::N;

    loop {
        if !do_move(&mut facing, &mut x, &mut y, &mut map) {
            break;
        }
    }

    map.iter()
        .map(|y| y.iter().filter(|x| **x == 'X').count() as i64)
        .sum()
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
        let result = solve_part1(INPUT);

        assert_eq!(result, 41);
    }
}
