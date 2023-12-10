use crate::solution::Solution;
use crate::util::grid::{BackedGrid, Grid, GridPosition};
use crate::util::position::{Direction, Position, RotationalDirection};

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
type PreparedInput<'a> = BackedGrid<'a, u8>;

pub fn prepare(input: &str) -> PreparedInput<'_> {
    BackedGrid::from_data_and_row_separator(input.as_bytes(), b'\n')
}

fn calc_loop(grid: &PreparedInput) -> Vec<(GridPosition, Direction, Direction)> {
    let (start_pos, _) = grid
        .iter::<Element>()
        .find(|(_, elem)| *elem == Element::Start)
        .unwrap();

    let neighbours = [
        // Up
        start_pos.y() > 0
            && matches!(
                grid.get(&start_pos.with_direction(&Direction::Up)),
                Element::Vertical | Element::SouthEast | Element::SouthWest
            ),
        // Down
        start_pos.y() < grid.dimensions.0 - 1
            && matches!(
                grid.get(&start_pos.with_direction(&Direction::Down)),
                Element::Vertical | Element::NorthEast | Element::NorthWest
            ),
        // Right
        start_pos.x() < grid.dimensions.1 - 1
            && matches!(
                grid.get(&start_pos.with_direction(&Direction::Right)),
                Element::Horizontal | Element::NorthWest | Element::SouthWest
            ),
        // Left
        start_pos.x() > 0
            && matches!(
                grid.get(&start_pos.with_direction(&Direction::Left)),
                Element::Horizontal | Element::NorthEast | Element::SouthEast
            ),
    ];

    let (mut incoming_direction, mut outgoing_direction) = match neighbours {
        [true, true, false, false] => (Direction::Down, Direction::Down),
        [false, false, true, true] => (Direction::Right, Direction::Right),
        [true, false, true, false] => (Direction::Down, Direction::Right),
        [true, false, false, true] => (Direction::Down, Direction::Left),
        [false, true, true, false] => (Direction::Up, Direction::Right),
        [false, true, false, true] => (Direction::Up, Direction::Left),
        _ => panic!(),
    };

    let mut pos = start_pos;
    let mut visited = vec![];
    loop {
        visited.push((pos, incoming_direction, outgoing_direction));

        pos = pos.with_direction(&outgoing_direction);
        incoming_direction = outgoing_direction;
        outgoing_direction = match incoming_direction {
            Direction::Up => match grid.get(&pos) {
                Element::Start => break,
                Element::Vertical => Direction::Up,
                Element::SouthWest => Direction::Left,
                Element::SouthEast => Direction::Right,
                _ => panic!(),
            },
            Direction::Down => match grid.get(&pos) {
                Element::Start => break,
                Element::Vertical => Direction::Down,
                Element::NorthWest => Direction::Left,
                Element::NorthEast => Direction::Right,
                _ => panic!(),
            },
            Direction::Right => match grid.get(&pos) {
                Element::Start => break,
                Element::Horizontal => Direction::Right,
                Element::SouthWest => Direction::Down,
                Element::NorthWest => Direction::Up,
                _ => panic!(),
            },
            Direction::Left => match grid.get(&pos) {
                Element::Start => break,
                Element::Horizontal => Direction::Left,
                Element::NorthEast => Direction::Up,
                Element::SouthEast => Direction::Down,
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
                Some(RotationalDirection::Clockwise) => 1,
                Some(RotationalDirection::Anticlockwise) => -1,
            }
        })
        .sum();
    let rot_direction = match clockwise_count {
        4 => RotationalDirection::Clockwise,
        -4 => RotationalDirection::Anticlockwise,
        _ => panic!(),
    };

    let mut visited_set = Grid::from_dimensions(grid.dimensions, false);
    visited_set.extend(visited.iter().map(|(pos, _, _)| *pos));

    let mut enclosed_entries = Grid::from_dimensions(grid.dimensions, false);

    let mut search = |mut pos: GridPosition, direction_to_search: &Direction| {
        pos = pos.with_direction(direction_to_search);

        while !visited_set.contains(&pos) {
            enclosed_entries.set(&pos, true);
            pos = pos.with_direction(direction_to_search);
        }
    };

    visited
        .into_iter()
        .for_each(|(pos, incoming_direction, outgoing_direction)| {
            search(pos, &incoming_direction.with_rotation(&rot_direction));
            if incoming_direction != outgoing_direction {
                search(pos, &outgoing_direction.with_rotation(&rot_direction));
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
