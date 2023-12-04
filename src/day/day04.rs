use crate::solution::Solution;
use bstr::ByteSlice;
use rustc_hash::FxHashSet;
use winnow::ascii::{dec_uint, space1};
use winnow::combinator::separated;
use winnow::prelude::*;
use winnow::token::take_till0;

type PreparedInput = Vec<usize>;

fn parse_card(input: &mut &[u8]) -> PResult<usize> {
    (take_till0(':'), ':', space1).parse_next(input)?;

    let winning = separated(1.., dec_uint::<_, u8, _>, space1)
        .map(|v: Vec<_>| v.into_iter().collect::<FxHashSet<u8>>())
        .parse_next(input)?;

    " |".parse_next(input)?;

    let mut count = 0;
    loop {
        if space1::<_, ()>.parse_next(input).is_err() {
            break;
        };
        let num = dec_uint::<_, u8, _>.parse_next(input)?;
        if winning.contains(&num) {
            count += 1;
        }
    }

    Ok(count)
}

pub fn prepare(input: &str) -> PreparedInput {
    input
        .as_bytes()
        .lines()
        .map(|line| parse_card.parse(line).unwrap())
        .collect::<Vec<_>>()
}

pub fn solve_part1(input: &PreparedInput) -> u32 {
    input
        .iter()
        .map(|&count| {
            if count == 0 {
                return 0;
            }
            2u32.pow(count as u32 - 1)
        })
        .sum()
}

pub fn solve_part2(input: &PreparedInput) -> u32 {
    let mut copies = vec![1u32; input.len()];

    for (i, &count) in input.iter().enumerate() {
        for next in 0..count {
            copies[i + next + 1] += copies[i];
        }
    }

    copies.iter().sum()
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 6);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 13);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 30);
    }
}
