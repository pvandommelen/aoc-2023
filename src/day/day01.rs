use crate::solution::Solution;

type PreparedInput<'a> = Vec<&'a str>;

pub fn prepare(input: &str) -> PreparedInput<'_> {
    input.lines().collect()
}

pub fn solve_part1(input: &PreparedInput) -> u32 {
    input
        .iter()
        .map(|&line| {
            let line = line.as_bytes();
            let first_digit = line.iter().find(|c| c.is_ascii_digit()).unwrap() - b'0';
            let last_digit = line.iter().rev().find(|c| c.is_ascii_digit()).unwrap() - b'0';

            first_digit as u32 * 10 + last_digit as u32
        })
        .sum()
}

pub fn solve_part2(input: &PreparedInput) -> u32 {
    input
        .iter()
        .map(|&line| {
            let line = line.as_bytes();
            let mut i = 0;
            let first_digit = loop {
                let c = line[i];
                if c.is_ascii_digit() {
                    break c - b'0';
                }
                if line[i..].starts_with("one".as_bytes()) {
                    break 1;
                }
                if line[i..].starts_with("two".as_bytes()) {
                    break 2;
                }
                if line[i..].starts_with("three".as_bytes()) {
                    break 3;
                }
                if line[i..].starts_with("four".as_bytes()) {
                    break 4;
                }
                if line[i..].starts_with("five".as_bytes()) {
                    break 5;
                }
                if line[i..].starts_with("six".as_bytes()) {
                    break 6;
                }
                if line[i..].starts_with("seven".as_bytes()) {
                    break 7;
                }
                if line[i..].starts_with("eight".as_bytes()) {
                    break 8;
                }
                if line[i..].starts_with("nine".as_bytes()) {
                    break 9;
                }
                i += 1;
            };
            let mut i = line.len() - 1;
            let last_digit = loop {
                let c = line[i];
                if c.is_ascii_digit() {
                    break c - b'0';
                }
                let remaining_line = &line[..i + 1];
                if remaining_line.ends_with("one".as_bytes()) {
                    break 1;
                }
                if remaining_line.ends_with("two".as_bytes()) {
                    break 2;
                }
                if remaining_line.ends_with("three".as_bytes()) {
                    break 3;
                }
                if remaining_line.ends_with("four".as_bytes()) {
                    break 4;
                }
                if remaining_line.ends_with("five".as_bytes()) {
                    break 5;
                }
                if remaining_line.ends_with("six".as_bytes()) {
                    break 6;
                }
                if remaining_line.ends_with("seven".as_bytes()) {
                    break 7;
                }
                if remaining_line.ends_with("eight".as_bytes()) {
                    break 8;
                }
                if remaining_line.ends_with("nine".as_bytes()) {
                    break 9;
                }
                i -= 1;
            };

            first_digit as u32 * 10 + last_digit as u32
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

    const EXAMPLE_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 4);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 142);
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solve_part2(&prepare(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            )),
            281
        );
    }
}
