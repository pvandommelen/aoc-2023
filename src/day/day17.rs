use crate::solution::Solution;
use crate::util::grid::Grid;
use crate::util::position::Direction::{Down, Left, Right, Up};
use crate::util::position::{Dimensions, Direction, Position};
use crate::util::solver::{solve_fn_priority, NodeResult};
use bstr::ByteSlice;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;

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
    direction_counter: u8,
    heat_loss: u32,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_distance = self.position.manhattan_distance(&self.target);
        let other_distance = other.position.manhattan_distance(&other.target);

        (self.heat_loss + self_distance as u32)
            .cmp(&(other.heat_loss + other_distance as u32))
            .reverse()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_part<F>(grid: &PreparedInput, test_state: F) -> u32
where
    F: Fn(&State, &State) -> bool,
{
    let dimensions: Dimensions = grid.dimensions.into();

    let mut distances = FxHashMap::with_capacity_and_hasher(grid.size(), Default::default());

    solve_fn_priority(
        |stack, state| {
            if state.position == state.target {
                return NodeResult::Stop;
            }

            let mut attempt_move = |attempt_direction: Direction| {
                let direction_counter = if attempt_direction == state.direction {
                    state.direction_counter + 1
                } else {
                    1
                };

                if let Some(next) = state
                    .position
                    .checked_moved(&dimensions, &attempt_direction)
                {
                    let next_heat_loss = state.heat_loss + *grid.get(&next) as u32;

                    let next_state = State {
                        target: state.target,
                        position: next,
                        direction: attempt_direction,
                        direction_counter,
                        heat_loss: next_heat_loss,
                    };

                    if !test_state(state, &next_state) {
                        return;
                    }

                    match distances.entry((next, attempt_direction, direction_counter)) {
                        Entry::Occupied(mut entry) => {
                            if *entry.get() <= next_heat_loss {
                                return;
                            }
                            entry.insert(next_heat_loss.min(*entry.get()));
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(next_heat_loss);
                        }
                    }

                    stack.push(next_state);
                }
            };

            if state.direction != Left {
                attempt_move(Right);
            }
            if state.direction != Up {
                attempt_move(Down);
            }
            if state.direction != Right {
                attempt_move(Left);
            }
            if state.direction != Down {
                attempt_move(Up);
            }
            NodeResult::Next
        },
        vec![State {
            target: Position(dimensions.height() - 1, dimensions.width() - 1),
            position: Position(0, 0),
            direction: Right,
            direction_counter: 0,
            heat_loss: 0,
        }],
    )
    .heat_loss
}

pub fn solve_part1(grid: &PreparedInput) -> u32 {
    solve_part(grid, |_, state| state.direction_counter <= 3)
}

pub fn solve_part2(grid: &PreparedInput) -> u32 {
    solve_part(grid, |previous_state, state| {
        if state.position == state.target {
            return state.direction_counter >= 4 && state.direction_counter <= 10;
        }
        if previous_state.direction == state.direction {
            state.direction_counter <= 10
        } else {
            previous_state.direction_counter >= 4
        }
    })
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
