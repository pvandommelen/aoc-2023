use crate::day::day16::Element::{
    Empty, MirrorDown, MirrorUp, SplitterHorizontal, SplitterVertical,
};
use crate::solution::Solution;
use crate::util::grid::Grid;
use crate::util::position::{Direction, Position, RotationalDirection};
use crate::util::solver::solve_fn_push;
use rustc_hash::FxHashSet;
use std::usize;

pub enum Element {
    Empty,
    MirrorUp,
    MirrorDown,
    SplitterVertical,
    SplitterHorizontal,
}

type PreparedInput = Grid<Element>;

pub fn prepare(input: &str) -> PreparedInput {
    Grid::from_rows(input.lines().map(|line| {
        line.chars().map(|c| match c {
            '.' => Empty,
            '\\' => MirrorDown,
            '/' => MirrorUp,
            '|' => SplitterVertical,
            '-' => SplitterHorizontal,
            _ => panic!(),
        })
    }))
}

fn calc_energized_count(
    grid: &PreparedInput,
    direction: Direction,
    initial_position: Position,
) -> usize {
    let mut all_states = FxHashSet::<u16>::default();
    let mut energized = FxHashSet::<u16>::default();

    // Assert that positions would take up to 7 bits.
    assert!(grid.dimensions.0 < 128);
    assert!(grid.dimensions.1 < 128);

    let dimensions = grid.dimensions.into();

    solve_fn_push(
        |stack, (dir, pos)| {
            energized.insert((pos.0 << 8) as u16 + pos.1 as u16);
            if !all_states.insert((pos.0 << 9) as u16 + (pos.1 << 2) as u16 + *dir as u8 as u16) {
                return;
            }

            let mut arr = [Direction::Up; 2];
            let mut slice = &mut arr[..];

            match (grid.get(pos), dir) {
                (Empty, _)
                | (SplitterHorizontal, Direction::Left | Direction::Right)
                | (SplitterVertical, Direction::Up | Direction::Down) => {
                    slice = &mut slice[0..1];
                    slice[0] = *dir;
                }
                (SplitterHorizontal, Direction::Up | Direction::Down) => {
                    slice[0] = Direction::Left;
                    slice[1] = Direction::Right;
                }
                (SplitterVertical, Direction::Left | Direction::Right) => {
                    slice[0] = Direction::Up;
                    slice[1] = Direction::Down;
                }
                (MirrorUp, Direction::Left | Direction::Right) => {
                    slice = &mut slice[0..1];
                    slice[0] = dir.with_rotation(&RotationalDirection::Anticlockwise);
                }
                (MirrorUp, Direction::Up | Direction::Down) => {
                    slice = &mut slice[0..1];
                    slice[0] = dir.with_rotation(&RotationalDirection::Clockwise);
                }
                (MirrorDown, Direction::Left | Direction::Right) => {
                    slice = &mut slice[0..1];
                    slice[0] = dir.with_rotation(&RotationalDirection::Clockwise);
                }
                (MirrorDown, Direction::Up | Direction::Down) => {
                    slice = &mut slice[0..1];
                    slice[0] = dir.with_rotation(&RotationalDirection::Anticlockwise);
                }
            };

            slice
                .iter()
                .filter_map(move |direction| {
                    pos.checked_moved(&dimensions, direction)
                        .map(|pos| (*direction, pos))
                })
                .for_each(|entry| {
                    stack.push(entry);
                });
        },
        vec![(direction, initial_position)],
    );

    energized.len()
}

pub fn solve_part1(grid: &PreparedInput) -> usize {
    calc_energized_count(grid, Direction::Right, Position::from_yx(0, 0))
}

pub fn solve_part2(grid: &PreparedInput) -> usize {
    let mut max = 0;
    for i in 0..grid.dimensions.1 {
        max = max.max(calc_energized_count(
            grid,
            Direction::Down,
            Position::from_yx(0, i),
        ));
        max = max.max(calc_energized_count(
            grid,
            Direction::Up,
            Position::from_yx(grid.dimensions.0 - 1, i),
        ));
    }
    for j in 0..grid.dimensions.0 {
        max = max.max(calc_energized_count(
            grid,
            Direction::Right,
            Position::from_yx(j, 0),
        ));
        max = max.max(calc_energized_count(
            grid,
            Direction::Left,
            Position::from_yx(j, grid.dimensions.1 - 1),
        ));
    }

    max
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).dimensions, (10, 10));
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 46);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 51);
    }
}
