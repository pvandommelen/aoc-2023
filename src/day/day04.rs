use crate::solution::Solution;
use bstr::ByteSlice;
use rustc_hash::FxHashSet;
use winnow::ascii::{dec_uint, space1};
use winnow::combinator::{iterator, terminated};
use winnow::error::ContextError;
use winnow::prelude::*;
use winnow::token::take_till0;

fn parse_card<'a>() -> impl Parser<&'a [u8], usize, ContextError> {
    let mut scratch = FxHashSet::<u8>::with_hasher(Default::default());
    move |input: &mut &[u8]| -> PResult<usize> {
        scratch.clear();

        (take_till0(':'), ':', space1).parse_next(input)?;

        let mut winning_it = iterator(*input, terminated(dec_uint::<_, u8, ContextError>, space1));
        winning_it.for_each(|elem| {
            scratch.insert(elem);
        });
        (*input, _) = winning_it.finish()?;

        "|".parse_next(input)?;

        let mut count = 0;
        loop {
            if space1::<_, ()>.parse_next(input).is_err() {
                break;
            };
            let num = dec_uint::<_, u8, _>.parse_next(input)?;
            if scratch.contains(&num) {
                count += 1;
            }
        }

        Ok(count)
    }
}

pub fn prepare(input: &str) -> impl Iterator<Item = usize> + '_ {
    let mut parse_card = parse_card();
    input
        .as_bytes()
        .lines()
        .map(move |line| parse_card.parse(line).unwrap())
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);

    let mut copies = vec![];

    let mut part1 = 0;
    input.enumerate().for_each(|(i, count)| {
        if count > 0 {
            part1 += 2u32.pow(count as u32 - 1);
        }
        let copies_len = copies.len();
        copies.resize(copies_len.max(i + count + 1), 1);
        for next in (i + 1)..(i + 1 + count) {
            copies[next] += copies[i];
        }
    });
    let part2: u32 = copies.iter().sum();

    (part1.into(), part2.into())
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
        assert_eq!(prepare(EXAMPLE_INPUT).count(), 6);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve(EXAMPLE_INPUT).0, 13u32.into());
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve(EXAMPLE_INPUT).1, 30u32.into());
    }
}
