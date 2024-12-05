use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part1(input);
    dbg!(output);
}

#[derive(Debug, EnumIter, PartialEq)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SO,
    O,
    NO,
}

fn solve_part1(input: &str) -> i64 {
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut count = 0;

    for (row_idx, row) in input.lines().enumerate() {
        board.push(Vec::new());
        for char in row.chars() {
            board[row_idx].push(char);
        }
    }

    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, char) in row.iter().enumerate() {
            if char == &'X' {
                for direction in Direction::iter() {
                    if process_char(direction, row_idx, col_idx, &board) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn process_char(direction: Direction, row_idx: usize, col_idx: usize, board: &[Vec<char>]) -> bool {
    let (mut is_n, mut is_e, mut is_s, mut is_o) = (false, false, false, false);

    if [Direction::N, Direction::NE, Direction::NO].contains(&direction) {
        if row_idx < 3 {
            return false;
        }

        is_n = true;
    }

    if [Direction::E, Direction::NE, Direction::SE].contains(&direction) {
        if col_idx > board[0].len() - 4 {
            return false;
        }

        is_e = true;
    }

    if [Direction::S, Direction::SE, Direction::SO].contains(&direction) {
        if row_idx > board.len() - 4 {
            return false;
        }

        is_s = true;
    }

    if [Direction::O, Direction::NO, Direction::SO].contains(&direction) {
        if col_idx < 3 {
            return false;
        }

        is_o = true;
    }

    let new_row = |nb: usize| {
        if is_n {
            row_idx - nb
        } else if is_s {
            row_idx + nb
        } else {
            row_idx
        }
    };

    let new_col = |nb: usize| {
        if is_o {
            col_idx - nb
        } else if is_e {
            col_idx + nb
        } else {
            col_idx
        }
    };

    if board[new_row(1)][new_col(1)] == 'M'
        && board[new_row(2)][new_col(2)] == 'A'
        && board[new_row(3)][new_col(3)] == 'S'
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn it_works() {
        let result = solve_part1(INPUT);

        assert_eq!(result, 18);
    }
}
