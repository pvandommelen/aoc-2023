use crate::day::day10::Direction::{Down, Left, Right, Up};
use crate::day::day10::RotationalDirection::{Anticlockwise, Clockwise};
use crate::solution::Solution;
use crate::util::grid::{BackedGrid, Grid};

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

impl From<&u8> for Element {
    fn from(value: &u8) -> Self {
        match *value {
            b'.' => Element::None,
            b'S' => Element::Start,
            b'|' => Element::Vertical,
            b'-' => Element::Horizontal,
            b'L' => Element::NorthEast,
            b'J' => Element::NorthWest,
            b'7' => Element::SouthWest,
            b'F' => Element::SouthEast,
            b'\n' => panic!("newline"),
            _ => panic!(),
        }
    }
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
            (Up, Clockwise) => Right,
            (Up, Anticlockwise) => Left,
            (Down, Clockwise) => Left,
            (Down, Anticlockwise) => Right,
            (Right, Clockwise) => Down,
            (Right, Anticlockwise) => Up,
            (Left, Clockwise) => Up,
            (Left, Anticlockwise) => Down,
        }
    }

    #[must_use]
    fn evolve_position(&self, position: &(usize, usize)) -> (usize, usize) {
        match self {
            Up => (position.0 - 1, position.1),
            Down => (position.0 + 1, position.1),
            Right => (position.0, position.1 + 1),
            Left => (position.0, position.1 - 1),
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
    fn from_incoming_and_outgoing(incoming: &Direction, outgoing: &Direction) -> Option<Self> {
        match (*outgoing as u8 + 4 - *incoming as u8) % 4 {
            0 => None,
            1 => Some(Clockwise),
            2 => None,
            3 => Some(Anticlockwise),
            _ => panic!(),
        }
    }
}

type PreparedInput<'a> = BackedGrid<'a, u8>;

pub fn prepare(input: &str) -> PreparedInput<'_> {
    BackedGrid::from_data_and_row_separator(input.as_bytes(), b'\n')
}

fn calc_loop(grid: &PreparedInput) -> Vec<((usize, usize), Direction, Direction)> {
    let (start_pos, _) = grid
        .iter::<Element>()
        .find(|(_, elem)| *elem == Element::Start)
        .unwrap();

    let neighbours = [
        // Up
        start_pos.0 > 0
            && matches!(
                grid.get(&Up.evolve_position(&start_pos)),
                Element::Vertical | Element::SouthEast | Element::SouthWest
            ),
        // Down
        start_pos.0 < grid.dimensions.0 - 1
            && matches!(
                grid.get(&Down.evolve_position(&start_pos)),
                Element::Vertical | Element::NorthEast | Element::NorthWest
            ),
        // Right
        start_pos.1 < grid.dimensions.1 - 1
            && matches!(
                grid.get(&Right.evolve_position(&start_pos)),
                Element::Horizontal | Element::NorthWest | Element::SouthWest
            ),
        // Left
        start_pos.1 > 0
            && matches!(
                grid.get(&Left.evolve_position(&start_pos)),
                Element::Horizontal | Element::NorthEast | Element::SouthEast
            ),
    ];

    let (mut incoming_direction, mut outgoing_direction) = match neighbours {
        [true, true, false, false] => (Down, Down),
        [false, false, true, true] => (Right, Right),
        [true, false, true, false] => (Down, Right),
        [true, false, false, true] => (Down, Left),
        [false, true, true, false] => (Up, Right),
        [false, true, false, true] => (Up, Left),
        _ => panic!(),
    };

    let mut pos = start_pos;
    let mut visited = vec![];
    loop {
        visited.push((pos, incoming_direction, outgoing_direction));

        pos = outgoing_direction.evolve_position(&pos);
        incoming_direction = outgoing_direction;
        outgoing_direction = match incoming_direction {
            Up => match grid.get(&pos) {
                Element::Start => break,
                Element::Vertical => Up,
                Element::SouthWest => Left,
                Element::SouthEast => Right,
                _ => panic!(),
            },
            Down => match grid.get(&pos) {
                Element::Start => break,
                Element::Vertical => Down,
                Element::NorthWest => Left,
                Element::NorthEast => Right,
                _ => panic!(),
            },
            Right => match grid.get(&pos) {
                Element::Start => break,
                Element::Horizontal => Right,
                Element::SouthWest => Down,
                Element::NorthWest => Up,
                _ => panic!(),
            },
            Left => match grid.get(&pos) {
                Element::Start => break,
                Element::Horizontal => Left,
                Element::NorthEast => Up,
                Element::SouthEast => Down,
                _ => panic!(),
            },
        };
    }
    visited
}

pub fn solve_part1(grid: &PreparedInput) -> usize {
    solve_parts(grid).0
}

pub fn solve_part2(grid: &PreparedInput) -> usize {
    solve_parts(grid).1
}

fn solve_parts(grid: &PreparedInput) -> (usize, usize) {
    let visited = calc_loop(grid);
    let part1_result = visited.len() / 2;

    let clockwise_count = visited
        .iter()
        .map(|(_, incoming_direction, outgoing_direction)| {
            match RotationalDirection::from_incoming_and_outgoing(
                incoming_direction,
                outgoing_direction,
            ) {
                None => 0,
                Some(Clockwise) => 1,
                Some(Anticlockwise) => -1,
            }
        })
        .sum();
    let rot_direction = match clockwise_count {
        4 => Clockwise,
        -4 => Anticlockwise,
        _ => panic!(),
    };

    let mut visited_set = Grid::from_dimensions(grid.dimensions, false);
    visited_set.extend(visited.iter().map(|(pos, _, _)| *pos));

    let mut enclosed_entries = Grid::from_dimensions(grid.dimensions, false);

    let mut search = |mut pos: (usize, usize), direction_to_search: Direction| {
        direction_to_search.apply_to_position(&mut pos);

        while !visited_set.contains(&pos) {
            enclosed_entries.set(&pos, true);
            direction_to_search.apply_to_position(&mut pos);
        }
    };

    visited
        .into_iter()
        .for_each(|(pos, incoming_direction, outgoing_direction)| {
            search(pos, incoming_direction.rotate(rot_direction));
            if incoming_direction != outgoing_direction {
                search(pos, outgoing_direction.rotate(rot_direction));
            }
        });

    (part1_result, enclosed_entries.count())
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    let (a, b) = solve_parts(&input);

    (a.into(), b.into())
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
