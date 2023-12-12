use crate::day::day12::Condition::{Broken, Operational, Unknown};
use crate::solution::Solution;
use bstr::ByteSlice;
use winnow::ascii::dec_uint;
use winnow::combinator::{alt, repeat, separated, separated_pair};
use winnow::prelude::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Condition {
    Broken,
    Operational,
    Unknown,
}

type PreparedInput = Vec<(Vec<Condition>, Vec<u8>)>;

pub fn prepare(input: &str) -> PreparedInput {
    input
        .as_bytes()
        .lines()
        .map(|line| {
            separated_pair(
                repeat(
                    1..,
                    alt((
                        b'?'.value(Unknown),
                        b'#'.value(Broken),
                        b'.'.value(Operational),
                    )),
                ),
                ' ',
                separated(1.., dec_uint::<_, u8, ()>, ','),
            )
            .parse(line)
            .unwrap()
        })
        .collect()
}

fn calc_arrangement_count(line: &[Condition], expected: &[u8]) -> usize {
    fn inner<'a>(
        cache: &mut Vec<Vec<usize>>,
        line: &'a [Condition],
        expected: &'a [u8],
        expected_total: usize,
    ) -> usize {
        if expected.is_empty() {
            // No more broken springs expected.
            if !line.contains(&Broken) {
                return 1;
            }
            return 0;
        }
        let nonop_index = line.iter().position(|c| *c != Operational);
        if nonop_index.is_none() {
            // No more broken springs possible, but it is expected.
            return 0;
        }
        let cached = cache[expected.len() - 1][line.len() - 1];
        if cached != 0 {
            return cached - 1;
        }
        let line = &line[nonop_index.unwrap()..];
        let next_length = expected[0] as usize;
        let possible = line.len() >= next_length
            && line[0..next_length]
                .iter()
                .all(|c| matches!(*c, Broken | Unknown))
            && line
                .get(next_length)
                .map_or(true, |c| matches!(*c, Operational | Unknown));

        let mut sum = 0;
        if possible {
            sum += inner(
                cache,
                &line[(next_length + 1).min(line.len())..],
                &expected[1..],
                expected_total - expected[0] as usize,
            );
        }
        if line[0] == Unknown {
            let line = &line[1..];
            if line.len() >= expected_total + expected.len() - 1 {
                sum += inner(cache, line, expected, expected_total);
            }
        }
        cache[expected.len() - 1][line.len() - 1] = sum + 1;
        sum
    }
    inner(
        &mut vec![vec![0; line.len()]; expected.len() + 1],
        line,
        expected,
        expected.iter().sum::<u8>() as usize,
    )
}

pub fn solve_part1(input: &PreparedInput) -> usize {
    input
        .iter()
        .map(|(line, expected)| calc_arrangement_count(line, expected))
        .sum()
}

pub fn solve_part2(input: &PreparedInput) -> usize {
    input
        .iter()
        .map(|(line, expected)| {
            let q = vec![Unknown];
            let line = [line, &q, line, &q, line, &q, line, &q, line]
                .into_iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            let expected = [expected, expected, expected, expected, expected]
                .into_iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            calc_arrangement_count(&line, &expected)
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

    const EXAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 6);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 21);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 525152);
    }
}
