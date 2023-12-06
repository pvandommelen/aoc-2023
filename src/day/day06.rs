use crate::solution::Solution;
use winnow::ascii::{dec_uint, digit1, space1};
use winnow::combinator::{preceded, separated, separated_pair};
use winnow::prelude::*;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Race {
    time_ms: u64,
    distance_mm: u64,
}

pub fn prepare_part1(input: &str) -> Vec<Race> {
    separated_pair(
        preceded(
            ("Time:", space1),
            separated(1.., dec_uint::<_, u64, ()>, space1),
        ),
        '\n',
        preceded(
            ("Distance:", space1),
            separated(1.., dec_uint::<_, u64, ()>, space1),
        ),
    )
    .map(|(times, distances): (Vec<_>, Vec<_>)| {
        times
            .into_iter()
            .zip(distances)
            .map(|(time_ms, distance_mm)| Race {
                time_ms,
                distance_mm,
            })
            .collect::<Vec<_>>()
    })
    .parse(input)
    .unwrap()
}

pub fn prepare_part2(input: &str) -> Race {
    separated_pair(
        preceded(
            ("Time:", space1::<_, ()>),
            separated(1.., digit1, space1).map(|s: String| s.parse().unwrap()),
        ),
        '\n',
        preceded(
            ("Distance:", space1::<_, ()>),
            separated(1.., digit1, space1).map(|s: String| s.parse().unwrap()),
        ),
    )
    .map(|(time_ms, distance_mm): (u64, u64)| Race {
        time_ms,
        distance_mm,
    })
    .parse(input)
    .unwrap()
}

pub fn solve_part(input: &[Race]) -> usize {
    input
        .iter()
        .map(|game| {
            (1..game.time_ms)
                .filter(|hold_time| hold_time * (game.time_ms - hold_time) > game.distance_mm)
                .count()
        })
        .reduce(|acc, c| acc * c)
        .unwrap()
}

pub fn solve(input: &str) -> (Solution, Solution) {
    (
        solve_part(&prepare_part1(input)).into(),
        solve_part(&[prepare_part2(input)]).into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    #[test]
    fn example_prepare_part1() {
        assert_eq!(prepare_part1(EXAMPLE_INPUT).len(), 3);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part(&prepare_part1(EXAMPLE_INPUT)), 288);
    }
    #[test]
    fn example_prepare_part2() {
        assert_eq!(
            prepare_part2(EXAMPLE_INPUT),
            Race {
                time_ms: 71530,
                distance_mm: 940200,
            }
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part(&[prepare_part2(EXAMPLE_INPUT)]), 71503);
    }
}
