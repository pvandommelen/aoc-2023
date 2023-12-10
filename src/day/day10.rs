use crate::day::day10::Direction::{Down, Left, Right, Up};
use crate::solution::Solution;
use crate::util::grid::Grid;
use rustc_hash::FxHashSet;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Element {
    None,
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn rotate(&self, rotational_direction: RotationalDirection) -> Self {
        match (self, rotational_direction) {
            (Up, RotationalDirection::Clockwise) => Right,
            (Up, RotationalDirection::Anticlockwise) => Left,
            (Down, RotationalDirection::Clockwise) => Left,
            (Down, RotationalDirection::Anticlockwise) => Right,
            (Right, RotationalDirection::Clockwise) => Down,
            (Right, RotationalDirection::Anticlockwise) => Up,
            (Left, RotationalDirection::Clockwise) => Up,
            (Left, RotationalDirection::Anticlockwise) => Down,
        }
    }

    fn apply_to_position(&self, position: &mut (usize, usize)) {
        match self {
            Up => position.0 -= 1,
            Down => position.0 += 1,
            Right => position.1 += 1,
            Left => position.1 -= 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RotationalDirection {
    Clockwise,
    Anticlockwise,
}

impl RotationalDirection {
    fn from_incoming_and_outgoing(incoming: Direction, outgoing: Direction) -> Option<Self> {
        match (outgoing as u8 + 4 - incoming as u8) % 4 {
            0 => None,
            1 => Some(RotationalDirection::Clockwise),
            2 => None,
            3 => Some(RotationalDirection::Anticlockwise),
            _ => panic!(),
        }
    }
}

type PreparedInput = Grid<Element>;

pub fn prepare(input: &str) -> Grid<Element> {
    Grid::from_rows(input.lines().map(|line| {
        line.chars().map(|c| match c {
            '.' => Element::None,
            'S' => Element::Start,
            '|' => Element::Vertical,
            '-' => Element::Horizontal,
            'L' => Element::NorthEast,
            'J' => Element::NorthWest,
            '7' => Element::SouthWest,
            'F' => Element::SouthEast,
            _ => panic!(),
        })
    }))
}

fn calc_loop(
    grid: &PreparedInput,
) -> (
    Vec<((usize, usize), Direction, Direction)>,
    RotationalDirection,
) {
    let (start_pos, _) = grid
        .iter()
        .find(|(_, elem)| **elem == Element::Start)
        .unwrap();

    let neighbours = [
        // Up
        start_pos.0 > 0
            && matches!(
                grid[start_pos.0 - 1][start_pos.1],
                Element::Vertical | Element::SouthEast | Element::SouthWest
            ),
        // Down
        start_pos.0 < grid.dimensions.0 - 1
            && matches!(
                grid[start_pos.0 + 1][start_pos.1],
                Element::Vertical | Element::NorthEast | Element::NorthWest
            ),
        // Right
        start_pos.1 < grid.dimensions.1 - 1
            && matches!(
                grid[start_pos.0][start_pos.1 + 1],
                Element::Horizontal | Element::NorthWest | Element::SouthWest
            ),
        // Left
        start_pos.1 > 0
            && matches!(
                grid[start_pos.0][start_pos.1 - 1],
                Element::Horizontal | Element::NorthEast | Element::SouthEast
            ),
    ];

    let (incoming_direction, outgoing_direction) = match neighbours {
        [true, true, false, false] => (Direction::Down, Direction::Down),
        [false, false, true, true] => (Direction::Right, Direction::Right),
        [true, false, true, false] => (Direction::Down, Direction::Right),
        [true, false, false, true] => (Direction::Down, Direction::Left),
        [false, true, true, false] => (Direction::Up, Direction::Right),
        [false, true, false, true] => (Direction::Up, Direction::Left),
        _ => panic!(),
    };

    let rotational_direction =
        RotationalDirection::from_incoming_and_outgoing(incoming_direction, outgoing_direction);
    let (mut clockwise_count, mut anticlockwise_count) = match rotational_direction {
        None => (0, 0),
        Some(RotationalDirection::Clockwise) => (1, 0),
        Some(RotationalDirection::Anticlockwise) => (0, 1),
    };
    let mut pos = start_pos;
    let mut visited = vec![(pos, incoming_direction, outgoing_direction)];
    let mut direction = outgoing_direction;
    loop {
        let (outgoing_pos, outgoing_direction) = match direction {
            Direction::Up => (
                (pos.0 - 1, pos.1),
                match grid[pos.0 - 1][pos.1] {
                    Element::Start => break,
                    Element::Vertical => Direction::Up,
                    Element::SouthWest => {
                        anticlockwise_count += 1;
                        Direction::Left
                    }
                    Element::SouthEast => {
                        clockwise_count += 1;
                        Direction::Right
                    }
                    _ => panic!(),
                },
            ),
            Direction::Down => (
                (pos.0 + 1, pos.1),
                match grid[pos.0 + 1][pos.1] {
                    Element::Start => break,
                    Element::Vertical => Direction::Down,
                    Element::NorthWest => {
                        clockwise_count += 1;
                        Direction::Left
                    }
                    Element::NorthEast => {
                        anticlockwise_count += 1;
                        Direction::Right
                    }
                    _ => panic!(),
                },
            ),
            Direction::Right => (
                (pos.0, pos.1 + 1),
                match grid[pos.0][pos.1 + 1] {
                    Element::Start => break,
                    Element::Horizontal => Direction::Right,
                    Element::SouthWest => {
                        clockwise_count += 1;
                        Direction::Down
                    }
                    Element::NorthWest => {
                        anticlockwise_count += 1;
                        Direction::Up
                    }
                    _ => panic!(),
                },
            ),
            Direction::Left => (
                (pos.0, pos.1 - 1),
                match grid[pos.0][pos.1 - 1] {
                    Element::Start => break,
                    Element::Horizontal => Direction::Left,
                    Element::NorthEast => {
                        clockwise_count += 1;
                        Direction::Up
                    }
                    Element::SouthEast => {
                        anticlockwise_count += 1;
                        Direction::Down
                    }
                    _ => panic!(),
                },
            ),
        };
        visited.push((outgoing_pos, direction, outgoing_direction));
        pos = outgoing_pos;
        direction = outgoing_direction;
    }
    let rot_direction = if clockwise_count > anticlockwise_count {
        assert_eq!(clockwise_count - anticlockwise_count, 4);
        RotationalDirection::Clockwise
    } else {
        assert_eq!(anticlockwise_count - clockwise_count, 4);
        RotationalDirection::Anticlockwise
    };
    (visited, rot_direction)
}

pub fn solve_part1(grid: &PreparedInput) -> usize {
    let (visited, _) = calc_loop(grid);
    visited.len() / 2
}

pub fn solve_part2(grid: &PreparedInput) -> usize {
    let (visited, rot_direction) = calc_loop(grid);
    let visited_set = visited
        .iter()
        .map(|(pos, _, _)| *pos)
        .collect::<FxHashSet<(usize, usize)>>();

    let mut enclosed_entries = FxHashSet::default();

    let mut search = |mut pos: (usize, usize), direction_to_search: Direction| {
        direction_to_search.apply_to_position(&mut pos);

        while !visited_set.contains(&pos) {
            enclosed_entries.insert(pos);
            direction_to_search.apply_to_position(&mut pos);
        }
    };

    visited
        .into_iter()
        .for_each(|(pos, incoming_direction, outgoing_direction)| {
            search(pos, incoming_direction.rotate(rot_direction));
            search(pos, outgoing_direction.rotate(rot_direction));
        });
    enclosed_entries.len()
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    const EXAMPLE2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE1).dimensions, (5, 5));
        assert_eq!(prepare(EXAMPLE2).dimensions, (5, 5));
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE1)), 4);
        assert_eq!(solve_part1(&prepare(EXAMPLE2)), 8);
    }

    const EXAMPLE3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    const EXAMPLE4: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    const EXAMPLE5: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    const EXAMPLE6: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE1)), 1);
        assert_eq!(solve_part2(&prepare(EXAMPLE2)), 1);
        assert_eq!(solve_part2(&prepare(EXAMPLE3)), 4);
        assert_eq!(solve_part2(&prepare(EXAMPLE4)), 4);
        assert_eq!(solve_part2(&prepare(EXAMPLE5)), 8);
        assert_eq!(solve_part2(&prepare(EXAMPLE6)), 10);
    }
}
