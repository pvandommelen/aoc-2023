use crate::solution::Solution;
use crate::util::grid::Grid;

type PreparedInput = Vec<Grid<bool>>;

pub fn prepare(input: &str) -> PreparedInput {
    input
        .split("\n\n")
        .map(|block| {
            Grid::from_rows(block.lines().map(|line| {
                line.chars().map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!(),
                })
            }))
        })
        .collect()
}

fn find_mirror_index(grid: &Grid<bool>) -> Option<usize> {
    for i in 1..grid.dimensions.0 {
        let valid = (0..i).rev().zip(i..grid.dimensions.0).all(|(a, b)| {
            let a = grid.get_row(a);
            let b = grid.get_row(b);
            a == b
        });
        if valid {
            return Some(i);
        }
    }
    None
}

fn find_mirror_index_with_single_allowed_error(grid: &Grid<bool>) -> Option<usize> {
    for i in 1..grid.dimensions.0 {
        let mut error_encountered = false;
        let valid = (0..i).rev().zip(i..grid.dimensions.0).all(|(a, b)| {
            let a = grid.get_row(a);
            let b = grid.get_row(b);
            let error_count = a.iter().zip(b.iter()).filter(|(a, b)| a != b).count();
            match error_count {
                0 => true,
                1 => {
                    if error_encountered {
                        false
                    } else {
                        error_encountered = true;
                        true
                    }
                }
                _ => false,
            }
        });
        if valid && error_encountered {
            return Some(i);
        }
    }
    None
}

pub fn solve_part1(input: &PreparedInput) -> usize {
    input
        .iter()
        .map(|grid| {
            if let Some(j) = find_mirror_index(grid) {
                return j * 100;
            }
            let transposed = grid.transposed();
            if let Some(i) = find_mirror_index(&transposed) {
                return i;
            }
            panic!("{}", grid)
        })
        .sum()
}

pub fn solve_part2(input: &PreparedInput) -> usize {
    input
        .iter()
        .map(|grid| {
            if let Some(j) = find_mirror_index_with_single_allowed_error(grid) {
                return j * 100;
            }
            let transposed = grid.transposed();
            if let Some(i) = find_mirror_index_with_single_allowed_error(&transposed) {
                return i;
            }
            panic!("{}", grid)
        })
        .sum()
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 2);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 405);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 400);
    }
}
