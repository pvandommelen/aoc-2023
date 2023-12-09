use crate::solution::Solution;
use rustc_hash::FxHashMap;
use winnow::ascii::alphanumeric1;
use winnow::combinator::{alt, delimited, repeat, separated, separated_pair};
use winnow::Parser;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}

type PreparedInput = (Vec<Direction>, FxHashMap<String, (String, String)>);

pub fn prepare(input: &str) -> PreparedInput {
    separated_pair(
        repeat(
            1..,
            alt(("L".value(Direction::Left), "R".value(Direction::Right))),
        ),
        "\n\n",
        separated(
            1..,
            separated_pair(
                alphanumeric1::<_, ()>.map(|s: &str| s.to_string()),
                " = ",
                delimited(
                    "(",
                    separated_pair(
                        alphanumeric1.map(|s: &str| s.to_string()),
                        ", ",
                        alphanumeric1.map(|s: &str| s.to_string()),
                    ),
                    ")",
                ),
            ),
            "\n",
        ),
    )
    .parse(input)
    .unwrap()
}

pub fn solve_part1(input: &PreparedInput) -> usize {
    let (directions, map) = input;

    let mut current = "AAA";
    directions
        .iter()
        .cycle()
        .position(|direction| {
            let (left, right) = &map[current];

            current = match direction {
                Direction::Left => left,
                Direction::Right => right,
            };
            current == "ZZZ"
        })
        .unwrap()
        + 1
}

/// Greatest common divisor calculation using Euclidean_algorithm (https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm)
fn gcd<T>(a: T, b: T) -> T
where
    T: Copy,
    T: Ord + Eq + std::ops::Rem<Output = T> + Default,
{
    let mut largest = a.max(b);
    let mut smallest = a.min(b);
    loop {
        if smallest == Default::default() {
            break largest;
        }
        (largest, smallest) = (smallest, largest % smallest);
    }
}

/// Least common multiple calculation using the GCD (https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor)
fn lcm<T>(a: T, b: T) -> T
where
    T: Copy,
    T: std::ops::Div<Output = T> + std::ops::Mul<Output = T>,
    T: Ord + Eq + std::ops::Rem<Output = T> + Default,
{
    b / gcd(a, b) * a
}

pub fn solve_part2(input: &PreparedInput) -> usize {
    let (directions, map) = input;

    map.keys()
        .filter(|key| key.ends_with('A'))
        .map(|mut current| {
            let mut encountered = FxHashMap::default();
            encountered.insert((0, current.to_owned()), 0);

            let mut encountered_z_distance = vec![];
            let (cycle_start, cycle_distance) = (0..)
                .find_map(|i| {
                    let direction_offset = i % directions.len();
                    let direction = directions[direction_offset];
                    let (left, right) = &map[current];

                    current = match direction {
                        Direction::Left => left,
                        Direction::Right => right,
                    };
                    if current.ends_with('Z') {
                        encountered_z_distance.push(i + 1);
                    }
                    encountered
                        .insert((direction_offset + 1, current.to_owned()), i + 1)
                        .map(|existing| (existing, i + 1))
                })
                .unwrap();

            (cycle_start, cycle_distance, encountered_z_distance)
        })
        .map(|(cycle_start, cycle_distance, encountered_z_distance)| {
            // For this problem it just so happens that the distance at which Z is encountered exactly matches the cycle size
            // This problem would be (significantly?) harder if that was not the case
            // Outside the example there is also just one valid Z distance found.
            // This further simplifies the problem to a simple least common multiple
            let z_distance = *encountered_z_distance.last().unwrap();
            assert_eq!(z_distance, cycle_distance - cycle_start);
            z_distance
        })
        .reduce(lcm)
        .unwrap()
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).0.len(), 2);
        assert_eq!(prepare(EXAMPLE_INPUT).1.len(), 7);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 2);
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solve_part2(&prepare(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            )),
            6
        );
    }
}
