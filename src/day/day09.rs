use crate::solution::Solution;
use winnow::ascii::dec_int;
use winnow::combinator::separated;
use winnow::prelude::*;

type PreparedInput = Vec<Vec<i32>>;

pub fn prepare(input: &str) -> PreparedInput {
    input
        .lines()
        .map(|line| {
            separated(1.., dec_int::<_, i32, ()>, " ")
                .parse(line)
                .unwrap()
        })
        .collect()
}

pub fn solve_both_parts(input: &PreparedInput) -> (i32, i32) {
    input
        .iter()
        .map(|sequence| -> (i32, i32) {
            let mut first_values = vec![];
            let mut last_values = vec![];
            let mut sequence = sequence.clone();
            loop {
                first_values.push(*sequence.first().unwrap());
                last_values.push(*sequence.last().unwrap());
                sequence = sequence
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect();
                if sequence.iter().all(|num| *num == 0) {
                    break (
                        last_values
                            .into_iter()
                            .rev()
                            .reduce(|num, last| num + last)
                            .unwrap(),
                        first_values
                            .into_iter()
                            .rev()
                            .reduce(|num, first| first - num)
                            .unwrap(),
                    );
                }
            }
        })
        .reduce(|(part1_sum, part2_sum), (part1, part2)| (part1_sum + part1, part2_sum + part2))
        .unwrap()
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    let output = solve_both_parts(&input);
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
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 3);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_both_parts(&prepare(EXAMPLE_INPUT)).0, 114);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_both_parts(&prepare(EXAMPLE_INPUT)).1, 2);
    }
}
