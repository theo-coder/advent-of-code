fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part2(input);
    dbg!(output);
}

fn solve_part2(input: &str) -> usize {
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

    for (i, nb_arr) in file_blocks.clone().iter().enumerate().rev() {
        if i % 2 == 0 {
            let qty_to_fit = nb_arr.len();

            for (j, pt_arr) in file_blocks.iter_mut().enumerate() {
                if j % 2 != 0 && j <= i && pt_arr.iter().filter(|c| *c == ".").count() >= qty_to_fit
                {
                    let first_pos = file_blocks[j].iter().position(|x| x == ".").unwrap();
                    file_blocks[j][first_pos..(qty_to_fit + first_pos)]
                        .clone_from_slice(&nb_arr[..qty_to_fit]);
                    file_blocks[i] = vec![".".to_string(); qty_to_fit];
                    break;
                }
            }
        }
    }

    file_blocks
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, c)| {
            if c.parse::<u64>().is_ok() {
                Some(c.parse::<u64>().unwrap() as usize * i)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn it_works() {
        let result = solve_part2(INPUT);

        assert_eq!(result, 2858);
    }
}
