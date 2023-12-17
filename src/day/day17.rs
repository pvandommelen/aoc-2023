use crate::solution::Solution;
use crate::util::grid::Grid;
use crate::util::position::Direction::{Down, Left, Right, Up};
use crate::util::position::{Dimensions, Direction, Position};
use crate::util::solver::{solve_fn_priority, NodeResult};
use bstr::ByteSlice;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::ops::RangeInclusive;

type PreparedInput = Grid<u8>;

pub fn prepare(input: &str) -> PreparedInput {
    Grid::from_rows(
        input
            .as_bytes()
            .lines()
            .map(|line| line.iter().map(|c| c - b'0')),
    )
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    target: Position,
    position: Position,
    direction: Direction,
    estimated_heat_loss: u32,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.estimated_heat_loss
            .cmp(&other.estimated_heat_loss)
            .reverse()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_part(grid: &PreparedInput, range: RangeInclusive<usize>) -> u32 {
    let dimensions: Dimensions = grid.dimensions.into();

    let mut distances = FxHashMap::with_capacity_and_hasher(grid.size(), Default::default());

    let target = Position(dimensions.height() - 1, dimensions.width() - 1);
    solve_fn_priority(
        |stack, state| {
            if state.position == state.target {
                return NodeResult::Stop;
            }

            let mut attempt_move = |attempt_direction: Direction| {
                let mut position = state.position;
                let mut next_estimated_heat_loss = state.estimated_heat_loss;

                let estimation_changed_by_direction = match attempt_direction {
                    Right | Down => 0,
                    Left | Up => 2,
                };

                for steps_moved in 1..=*range.end() {
                    position = match position.checked_moved(&dimensions, &attempt_direction) {
                        None => return,
                        Some(next) => next,
                    };
                    next_estimated_heat_loss +=
                        *grid.get(&position) as u32 + estimation_changed_by_direction - 1;

                    if steps_moved < *range.start() {
                        continue;
                    }
                    let next_state = State {
                        target: state.target,
                        position,
                        direction: attempt_direction,
                        estimated_heat_loss: next_estimated_heat_loss,
                    };

                    match distances.entry((position, attempt_direction)) {
                        Entry::Occupied(mut entry) => {
                            if *entry.get() <= next_estimated_heat_loss {
                                continue;
                            }
                            entry.insert(next_estimated_heat_loss);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(next_estimated_heat_loss);
                        }
                    }

                    stack.push(next_state);
                }
            };

            if state.direction != Left && state.direction != Right {
                attempt_move(Right);
            }
            if state.direction != Up && state.direction != Down {
                attempt_move(Down);
            }
            if state.direction != Right && state.direction != Left {
                attempt_move(Left);
            }
            if state.direction != Down && state.direction != Up {
                attempt_move(Up);
            }
            NodeResult::Next
        },
        vec![
            State {
                target,
                position: Position(0, 0),
                direction: Right,
                estimated_heat_loss: Position(0, 0).manhattan_distance(&target) as u32,
            },
            State {
                target,
                position: Position(0, 0),
                direction: Down,
                estimated_heat_loss: Position(0, 0).manhattan_distance(&target) as u32,
            },
        ],
    )
    .estimated_heat_loss
}

pub fn solve_part1(grid: &PreparedInput) -> u32 {
    solve_part(grid, 1..=3)
}

pub fn solve_part2(grid: &PreparedInput) -> u32 {
    solve_part(grid, 4..=10)
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).dimensions, (13, 13));
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 102);
    }
    #[test]
    fn example_part2a() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 94);
    }
    #[test]
    fn example_part2b() {
        assert_eq!(
            solve_part2(&prepare(
                "111111111111
999999999991
999999999991
999999999991
999999999991"
            )),
            71
        );
    }
}
