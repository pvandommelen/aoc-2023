use crate::solution::Solution;
use num::integer::lcm;
use rustc_hash::FxHashMap;
use winnow::ascii::alphanumeric1;
use winnow::combinator::{alt, delimited, repeat, separated, separated_pair};
use winnow::Parser;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Node(u32);

impl Node {
    fn from_u8_slice(slice: &[u8]) -> Self {
        assert_eq!(slice.len(), 3);
        Node(slice.iter().fold(0u32, |num, c| (num << 8) + *c as u32))
    }

    fn ends_with(&self, c: u8) -> bool {
        (self.0 & u8::MAX as u32) == c as u32
    }
}

type PreparedInput = (Vec<Direction>, FxHashMap<Node, (Node, Node)>);

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
                alphanumeric1::<_, ()>.map(Node::from_u8_slice),
                " = ",
                delimited(
                    "(",
                    separated_pair(
                        alphanumeric1.map(Node::from_u8_slice),
                        ", ",
                        alphanumeric1.map(Node::from_u8_slice),
                    ),
                    ")",
                ),
            ),
            "\n",
        ),
    )
    .parse(input.as_bytes())
    .unwrap()
}

pub fn solve_part1(input: &PreparedInput) -> usize {
    let (directions, map) = input;

    let mut current = Node::from_u8_slice("AAA".as_bytes());
    directions
        .iter()
        .cycle()
        .position(|direction| {
            let (left, right) = map[&current];

            current = match direction {
                Direction::Left => left,
                Direction::Right => right,
            };
            current == Node::from_u8_slice("ZZZ".as_bytes())
        })
        .unwrap()
        + 1
}

pub fn solve_part2(input: &PreparedInput) -> usize {
    let (directions, map) = input;

    map.keys()
        .filter(|key| key.ends_with(b'A'))
        .map(|mut current| {
            let mut encountered = vec![FxHashMap::default(); directions.len() + 1];
            encountered[0].insert(current.to_owned(), 0);

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
                    if current.ends_with(b'Z') {
                        encountered_z_distance.push(i + 1);
                    }
                    encountered[direction_offset + 1]
                        .insert(current.to_owned(), i + 1)
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
