use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part2(input);
    dbg!(output);
}

#[derive(Debug, EnumIter, PartialEq)]
enum Direction {
    E,
    S,
}

fn solve_part2(input: &str) -> i64 {
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
            if char == &'M' || char == &'S' {
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
    if row_idx > board.len() - 3 || col_idx > board[0].len() - 3 {
        return false;
    }

    let char = board[row_idx][col_idx];

    match char {
        'M' => match direction {
            Direction::E => {
                if board[row_idx][col_idx + 2] == 'M'
                    && board[row_idx + 1][col_idx + 1] == 'A'
                    && board[row_idx + 2][col_idx] == 'S'
                    && board[row_idx + 2][col_idx + 2] == 'S'
                {
                    return true;
                }
            }
            Direction::S => {
                if board[row_idx + 2][col_idx] == 'M'
                    && board[row_idx + 1][col_idx + 1] == 'A'
                    && board[row_idx][col_idx + 2] == 'S'
                    && board[row_idx + 2][col_idx + 2] == 'S'
                {
                    return true;
                }
            }
        },
        'S' => match direction {
            Direction::E => {
                if board[row_idx][col_idx + 2] == 'S'
                    && board[row_idx + 1][col_idx + 1] == 'A'
                    && board[row_idx + 2][col_idx] == 'M'
                    && board[row_idx + 2][col_idx + 2] == 'M'
                {
                    return true;
                }
            }
            Direction::S => {
                if board[row_idx + 2][col_idx] == 'S'
                    && board[row_idx + 1][col_idx + 1] == 'A'
                    && board[row_idx][col_idx + 2] == 'M'
                    && board[row_idx + 2][col_idx + 2] == 'M'
                {
                    return true;
                }
            }
        },
        _ => unreachable!(),
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
    fn part2() {
        let result = solve_part2(INPUT);

        assert_eq!(result, 9);
    }
}
