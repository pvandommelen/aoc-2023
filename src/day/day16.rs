use crate::day::day16::Element::{
    Empty, MirrorDown, MirrorUp, SplitterHorizontal, SplitterVertical,
};
use crate::solution::Solution;
use crate::util::grid::Grid;
use crate::util::position::{Direction, Position, RotationalDirection};
use crate::util::solver::solve_fn;
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
    let mut all_states = FxHashSet::<(Direction, Position)>::default();
    let mut energized = FxHashSet::<Position>::default();

    let dimensions = grid.dimensions.into();

    solve_fn(
        |(dir, pos)| {
            energized.insert(*pos);
            if !all_states.insert((*dir, *pos)) {
                return vec![];
            }

            let out_directions = match (grid.get(pos), dir) {
                (Empty, dir) => vec![*dir],
                (SplitterHorizontal, Direction::Left | Direction::Right) => {
                    vec![*dir]
                }
                (SplitterHorizontal, Direction::Up | Direction::Down) => {
                    vec![Direction::Left, Direction::Right]
                }
                (SplitterVertical, Direction::Left | Direction::Right) => {
                    vec![Direction::Up, Direction::Down]
                }
                (SplitterVertical, Direction::Up | Direction::Down) => {
                    vec![*dir]
                }
                (MirrorUp, Direction::Left | Direction::Right) => {
                    vec![dir.with_rotation(&RotationalDirection::Anticlockwise)]
                }
                (MirrorUp, Direction::Up | Direction::Down) => {
                    vec![dir.with_rotation(&RotationalDirection::Clockwise)]
                }
                (MirrorDown, Direction::Left | Direction::Right) => {
                    vec![dir.with_rotation(&RotationalDirection::Clockwise)]
                }
                (MirrorDown, Direction::Up | Direction::Down) => {
                    vec![dir.with_rotation(&RotationalDirection::Anticlockwise)]
                }
            };

            out_directions
                .into_iter()
                .filter_map(move |direction| {
                    pos.checked_moved(&dimensions, &direction)
                        .map(|pos| (direction, pos))
                })
                .collect()
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
