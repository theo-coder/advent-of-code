fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part1(input);
    dbg!(output);
}

fn solve_part1(input: &str) -> usize {
    let mut file_blocks = Vec::new();
    let mut block_idx = 0;

    for (i, nb) in input.trim_end().chars().enumerate() {
        if i % 2 == 0 {
            file_blocks.push(vec![
                block_idx.to_string();
                nb.to_digit(10).unwrap() as usize
            ]);
            block_idx += 1;
        } else {
            file_blocks.push(vec![".".to_string(); nb.to_digit(10).unwrap() as usize]);
        }
    }

    let mut file_blocks = file_blocks
        .iter()
        .flatten()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    while file_blocks.contains(&".".to_string()) {
        let last_char = file_blocks.pop().unwrap().to_string();
        if last_char == *".".to_owned() {
            continue;
        }

        let pt_idx = file_blocks.iter().position(|c| **c == '.'.to_string());

        if let Some(pt_idx) = pt_idx {
            file_blocks[pt_idx] = last_char;
        }
    }

    file_blocks
        .iter()
        .enumerate()
        .map(|(i, c)| c.parse::<u64>().unwrap() as usize * i)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn it_works() {
        let result = solve_part1(INPUT);

        assert_eq!(result, 1928);
    }
}
