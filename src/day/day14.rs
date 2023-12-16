use crate::day::day14::Element::{CubeShapedRock, Empty, RoundedRock};
use crate::solution::Solution;
use crate::util::grid::{CellDisplay, Grid};
use crate::util::position::{Direction, Position};
use rustc_hash::FxHashMap;
use std::fmt::{Formatter, Write};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Element {
    Empty,
    RoundedRock,
    CubeShapedRock,
}
impl CellDisplay for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Empty => '.',
            RoundedRock => 'O',
            CubeShapedRock => '#',
        })
    }
}

type PreparedInput = Grid<Element>;

pub fn prepare(input: &str) -> PreparedInput {
    Grid::from_rows(input.lines().map(|line| {
        line.chars().map(|c| match c {
            '.' => Empty,
            'O' => RoundedRock,
            '#' => CubeShapedRock,
            _ => panic!(),
        })
    }))
}

fn tilt(grid: &mut Grid<Element>, direction: Direction) {
    let position_iterator = grid.positions();
    let dimensions = grid.dimensions.into();
    let func = |pos: Position| {
        let value = grid.get(&pos);
        if *value == RoundedRock {
            let mut current_pos = pos;
            loop {
                let next_pos =
                    current_pos
                        .checked_moved(&dimensions, &direction)
                        .and_then(|next_pos| {
                            if matches!(grid.get(&next_pos), Empty) {
                                Some(next_pos)
                            } else {
                                None
                            }
                        });
                if next_pos.is_none() {
                    grid.set(&pos, Empty);
                    grid.set(&current_pos, RoundedRock);
                    break;
                }
                current_pos = next_pos.unwrap();
            }
        }
    };
    match direction {
        Direction::Up | Direction::Left => position_iterator.for_each(func),
        Direction::Down | Direction::Right => position_iterator.rev().for_each(func),
    };
}

fn calc_total_load(grid: &Grid<Element>) -> usize {
    grid.iter()
        .map(|(pos, value)| match value {
            RoundedRock => grid.dimensions.1 - pos.y(),
            _ => 0,
        })
        .sum()
}

pub fn solve_part1(input: &PreparedInput) -> usize {
    let mut grid = input.clone();
    tilt(&mut grid, Direction::Up);
    calc_total_load(&grid)
}

pub fn solve_part2(input: &PreparedInput) -> usize {
    let mut grid = input.clone();

    let mut map = FxHashMap::default();
    let mut total_loads = vec![];

    let cycles = 1000000000;
    for i in 0..cycles {
        tilt(&mut grid, Direction::Up);
        tilt(&mut grid, Direction::Left);
        tilt(&mut grid, Direction::Down);
        tilt(&mut grid, Direction::Right);
        total_loads.push(calc_total_load(&grid));

        if let Some(existing) = map.insert(grid.clone(), i) {
            let loop_length = i - existing;
            let remaining_loops = cycles - existing - 1;
            let cycles_after_loop_reached = remaining_loops % loop_length;
            return total_loads[existing + cycles_after_loop_reached];
        }
    }
    panic!()
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).dimensions, (10, 10));
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 136);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 64);
    }
}
