use crate::solution::Solution;
use bstr::ByteSlice;

type PreparedInput<'a> = Vec<&'a [u8]>;

pub fn prepare(input: &str) -> PreparedInput<'_> {
    input.as_bytes().lines().collect()
}

pub fn solve_part1(input: &PreparedInput) -> u32 {
    input
        .iter()
        .map(|&line| {
            let first_digit = line.iter().find(|c| c.is_ascii_digit()).unwrap() - b'0';
            let last_digit = line.iter().rev().find(|c| c.is_ascii_digit()).unwrap() - b'0';

            first_digit as u32 * 10 + last_digit as u32
        })
        .sum()
}

fn parse_num(line: &[u8]) -> Option<u8> {
    if line.len() >= 3 {
        if line.starts_with("one".as_bytes()) {
            return Some(1);
        }
        if line.starts_with("two".as_bytes()) {
            return Some(2);
        }
        if line.starts_with("six".as_bytes()) {
            return Some(6);
        }
    }
    if line.len() >= 4 {
        if line.starts_with("four".as_bytes()) {
            return Some(4);
        }
        if line.starts_with("five".as_bytes()) {
            return Some(5);
        }
        if line.starts_with("nine".as_bytes()) {
            return Some(9);
        }
    }
    if line.len() >= 5 {
        if line.starts_with("three".as_bytes()) {
            return Some(3);
        }
        if line.starts_with("seven".as_bytes()) {
            return Some(7);
        }
        if line.starts_with("eight".as_bytes()) {
            return Some(8);
        }
    }
    None
}

pub fn solve_part2(input: &PreparedInput) -> u32 {
    input
        .iter()
        .map(|&line| {
            // Extracting the duplicate closure results is significantly slower.
            let first_digit = line
                .iter()
                .enumerate()
                .find_map(|(i, c): (usize, &u8)| {
                    if c.is_ascii_digit() {
                        Some(*c - b'0')
                    } else {
                        parse_num(&line[i..])
                    }
                })
                .unwrap();
            let last_digit = line
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, c): (usize, &u8)| {
                    if c.is_ascii_digit() {
                        Some(*c - b'0')
                    } else {
                        parse_num(&line[i..])
                    }
                })
                .unwrap();

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
