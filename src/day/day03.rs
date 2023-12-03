use crate::solution::Solution;
use crate::util::grid::Grid;
use std::ops::Range;
use winnow::ascii::dec_uint;
use winnow::combinator::{delimited, separated};
use winnow::prelude::*;
use winnow::Located;

#[derive(Copy, Clone)]
pub enum Cell {
    None,
    Number(u8),
    Symbol,
    Gear,
}

type PreparedInput = (Vec<Vec<(u32, Range<usize>)>>, Grid<Cell>);

pub fn prepare(input: &str) -> PreparedInput {
    let numbers = input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            delimited(
                winnow::token::take_till0('0'..='9'),
                separated(
                    0..,
                    dec_uint::<_, u32, ()>.with_span(),
                    winnow::token::take_till1('0'..='9'),
                ),
                winnow::token::take_till0('0'..='9'),
            )
            .parse(Located::new(line))
            .unwrap()
        })
        .collect::<Vec<_>>();

    let grid = Grid::from_rows(input.lines().map(|line| {
        line.as_bytes().iter().map(|c| match c {
            b'0'..=b'9' => Cell::Number(c - b'0'),
            b'.' => Cell::None,
            b'*' => Cell::Gear,
            _ => Cell::Symbol,
        })
    }));
    (numbers, grid)
}

pub fn solve_part1(input: &PreparedInput) -> u32 {
    let (numbers, grid) = input;
    numbers
        .iter()
        .enumerate()
        .map(|(j, row)| {
            let j_range = j.saturating_sub(1)..=(j + 1).min(numbers.len() - 1);
            row.iter()
                .map(|(num, range)| {
                    let i_range =
                        range.start.saturating_sub(1)..=range.end.min(grid.dimensions.1 - 1);
                    for j in j_range.clone() {
                        for i in i_range.clone() {
                            if matches!(grid[j][i], Cell::Symbol | Cell::Gear) {
                                return *num;
                            }
                        }
                    }
                    0u32
                })
                .sum::<u32>()
        })
        .sum()
}

pub fn solve_part2(input: &PreparedInput) -> u32 {
    let (numbers, grid) = input;
    grid.iter()
        .map(|(pos, cell)| {
            if !matches!(cell, Cell::Gear) {
                return 0;
            }
            let j_range = pos.0.saturating_sub(1)..=(pos.0 + 1).min(grid.dimensions.0 - 1);
            // No fix is necessary on the bound in the x-direction because it is not used in indices.
            let i_range = pos.1.saturating_sub(1)..=(pos.1 + 1);

            let mut matching_numbers_mult = 1u32;
            let mut matching_numbers_count = 0u8;
            for j in j_range {
                let row_numbers = &numbers[j];
                for (num, range) in row_numbers {
                    if range.start <= *i_range.end() && range.end > *i_range.start() {
                        matching_numbers_mult = matching_numbers_mult.wrapping_mul(*num);
                        matching_numbers_count += 1;
                    }
                }
            }
            if matching_numbers_count != 2 {
                return 0;
            }
            matching_numbers_mult
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

    const EXAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    #[test]
    fn example_prepare() {
        let (numbers, grid) = prepare(EXAMPLE_INPUT);
        assert_eq!(numbers[0], vec![(467, 0..3), (114, 5..8)]);
        assert_eq!(numbers[1], vec![]);
        assert_eq!(numbers[2], vec![(35, 2..4), (633, 6..9)]);

        assert_eq!(grid.dimensions, (10, 10));
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 4361);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 467835);
    }
}
