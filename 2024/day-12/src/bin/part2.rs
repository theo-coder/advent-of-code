use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve_part2(input);
    dbg!(output);
}

type Corner = [((isize, isize), (isize, isize), (isize, isize)); 4];
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const CORNERS: Corner = [
    ((0, -1), (1, -1), (1, 0)),
    ((1, 0), (1, 1), (0, 1)),
    ((0, 1), (-1, 1), (-1, 0)),
    ((-1, 0), (-1, -1), (0, -1)),
];

fn solve_part2(input: &str) -> usize {
    let mut map = Vec::new();
    for (line_idx, line) in input.lines().enumerate() {
        map.push(Vec::new());
        for char in line.chars() {
            map[line_idx].push(char);
        }
    }

    let mut found_pos = HashSet::new();
    let mut regions = Vec::new();

    for y in 0..map.len() as isize {
        for x in 0..map[0].len() as isize {
            if found_pos.contains(&(x, y)) {
                continue;
            }

            let mut new_region = Vec::new();

            found_pos.insert((x, y));
            new_region.push((x, y));

            let mut investigate = Vec::new();

            for (dx, dy) in &DIRECTIONS {
                if char_at(&map, x + dx, y + dy) == char_at(&map, x, y) {
                    investigate.push((x + dx, y + dy));
                }
            }

            while let Some((ix, iy)) = investigate.pop() {
                if found_pos.contains(&(ix, iy)) {
                    continue;
                }

                found_pos.insert((ix, iy));
                new_region.push((ix, iy));

                for (dx, dy) in &DIRECTIONS {
                    if char_at(&map, ix + dx, iy + dy) == char_at(&map, ix, iy) {
                        investigate.push((ix + dx, iy + dy));
                    }
                }
            }

            regions.push(new_region);
        }
    }

    regions
        .iter()
        .map(|r| r.len() * calculate_perimeter(&map, r))
        .sum()
}

fn char_at(map: &[Vec<char>], x: isize, y: isize) -> Option<char> {
    map.get(y as usize)?.get(x as usize).copied()
}

fn calculate_perimeter(map: &[Vec<char>], plots: &[(isize, isize)]) -> usize {
    let mut perimeter = 0;
    let points: HashSet<_> = plots.iter().copied().collect();

    for &(x, y) in &points {
        for ((dx0, dy0), (dx1, dy1), (dx2, dy2)) in &CORNERS {
            if char_at(map, x + dx0, y + dy0) != char_at(map, x, y)
                && char_at(map, x + dx2, y + dy2) != char_at(map, x, y)
            {
                perimeter += 1;
            }

            if char_at(map, x + dx0, y + dy0) == char_at(map, x, y)
                && char_at(map, x + dx2, y + dy2) == char_at(map, x, y)
                && char_at(map, x + dx1, y + dy1) != char_at(map, x, y)
            {
                perimeter += 1;
            }
        }
    }

    perimeter
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works_1() {
        let input = "AAAA
BBCD
BBCC
EEEC";

        let result = solve_part2(input);

        assert_eq!(result, 80);
    }

    #[test]
    fn it_works_2() {
        let input = "OOO
OXOXO
OOOOO
OXOXO
OOOOO";

        let result = solve_part2(input);

        assert_eq!(result, 436);
    }

    #[test]
    fn it_works_3() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

        let result = solve_part2(input);

        assert_eq!(result, 236);
    }

    #[test]
    fn it_works_4() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        let result = solve_part2(input);

        assert_eq!(result, 368);
    }

    #[test]
    fn it_works_5() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        let result = solve_part2(input);

        assert_eq!(result, 1206);
    }
}
