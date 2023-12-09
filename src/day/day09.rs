use crate::solution::Solution;
use bstr::ByteSlice;
use winnow::ascii::dec_int;
use winnow::combinator::separated;
use winnow::prelude::*;

pub fn prepare(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.as_bytes().lines().map(|line| {
        separated(1.., dec_int::<_, i32, ()>, " ")
            .parse(line)
            .unwrap()
    })
}

pub fn solve_both_parts<Input>(input: Input) -> (i32, i32)
where
    Input: Iterator<Item = Vec<i32>>,
{
    input
        .map(|mut sequence| -> (i32, i32) {
            let mut first_value = sequence[0];
            let mut last_value = sequence[sequence.len() - 1];
            let mut i = 0;
            loop {
                for i in 0..sequence.len() - 1 {
                    sequence[i] = sequence[i + 1] - sequence[i];
                }
                sequence.pop();
                if sequence.iter().all(|num| *num == 0) {
                    break (last_value, first_value);
                }
                if i % 2 == 0 {
                    first_value -= sequence[0];
                } else {
                    first_value += sequence[0];
                }
                last_value += sequence[sequence.len() - 1];
                i += 1;
            }
        })
        .reduce(|(part1_sum, part2_sum), (part1, part2)| (part1_sum + part1, part2_sum + part2))
        .unwrap()
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    let output = solve_both_parts(input);
    (output.0.into(), output.1.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).count(), 3);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_both_parts(prepare(EXAMPLE_INPUT)).0, 114);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_both_parts(prepare(EXAMPLE_INPUT)).1, 2);
    }
}
