use crate::solution::Solution;
use std::ops::Range;
use winnow::ascii::dec_uint;
use winnow::combinator::{preceded, separated, separated_pair};
use winnow::prelude::*;
use winnow::token::take_till1;

#[derive(Copy, Clone)]
pub enum Cell {
    None,
    Number(u8),
    Symbol,
    Gear,
}

type PreparedInput = (Vec<u64>, Vec<Vec<(u64, u64, u64)>>);

pub fn prepare(input: &str) -> PreparedInput {
    separated_pair(
        preceded("seeds: ", separated(1.., dec_uint::<_, u64, ()>, ' ')),
        "\n\n",
        separated(
            1..,
            preceded(
                (take_till1('\n'), '\n'),
                separated(
                    1..,
                    (
                        dec_uint::<_, u64, ()>,
                        ' ',
                        dec_uint::<_, u64, ()>,
                        ' ',
                        dec_uint::<_, u64, ()>,
                    )
                        .map(|(a, _, b, _, c)| (a, b, c)),
                    '\n',
                )
                .map(|mut v: Vec<_>| {
                    v.sort_unstable_by_key(|(_, input_start, _)| *input_start);
                    v
                }),
            ),
            "\n\n",
        ),
    )
    .parse(input)
    .unwrap()
}

pub fn solve_part1(input: &PreparedInput) -> u64 {
    let (seeds, mappings) = input;
    let numbers = mappings.iter().fold(seeds.clone(), |numbers, mapping| {
        numbers
            .into_iter()
            .map(|previous| {
                mapping
                    .iter()
                    .find(|&&(_, input_start, length)| {
                        previous >= input_start && previous <= input_start + length
                    })
                    .map_or(previous, |map| previous - map.1 + map.0)
            })
            .collect()
    });
    numbers.into_iter().min().unwrap()
}

pub fn solve_part2(input: &PreparedInput) -> u64 {
    let (seeds, mappings) = input;
    let numbers = mappings.iter().fold(
        seeds
            .chunks_exact(2)
            .map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .collect::<Vec<_>>(),
        |numbers, mapping| {
            numbers
                .into_iter()
                .flat_map(|mut previous| {
                    let mut result = vec![];
                    for &(output_start, input_start, length) in mapping {
                        // As the number ranges are sorted, if we are fully beyond we can break the loop
                        if previous.end < input_start {
                            break;
                        }
                        if previous.start < input_start + length && previous.end > input_start {
                            if previous.start < input_start {
                                result.push(previous.start..input_start);
                                previous.start = input_start;
                            }
                            result.push(
                                previous.start - input_start + output_start
                                    ..previous.end.min(input_start + length) - input_start
                                        + output_start,
                            );
                            previous.start = input_start + length;
                        }
                    }
                    if previous.end > previous.start {
                        result.push(previous.clone());
                    }
                    result
                })
                .collect()
        },
    );
    numbers.into_iter().map(|range| range.start).min().unwrap()
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn example_prepare() {
        let (seeds, mapping) = prepare(EXAMPLE_INPUT);
        assert_eq!(seeds.len(), 4);
        assert_eq!(mapping.len(), 7);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 35);
    }
    #[test]
    fn example_part2_single() {
        let mut input = prepare(EXAMPLE_INPUT);
        input.0 = vec![82, 1];
        assert_eq!(solve_part2(&input), 46);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 46);
    }
}
