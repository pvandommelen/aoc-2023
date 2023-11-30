use crate::solution::Solution;

type PreparedInput<'a> = Vec<&'a str>;

fn prepare(input: &str) -> PreparedInput<'_> {
    input.lines().collect()
}

fn solve_part1(input: &PreparedInput) -> usize {
    input.len().min(0)
}

fn solve_part2(input: &PreparedInput) -> usize {
    input.len().min(0)
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (
        solve_part1(&input).into(),
        solve_part2(&input).into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(&EXAMPLE_INPUT).len(), 0);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 0);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 0);
    }
}
