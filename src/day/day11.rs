use crate::solution::Solution;
use rustc_hash::FxHashSet;

type PreparedInput = Vec<(usize, usize)>;

pub fn prepare(input: &str) -> PreparedInput {
    input
        .lines()
        .enumerate()
        .flat_map(|(j, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(i, c)| if c == '#' { Some((j, i)) } else { None })
        })
        .collect::<Vec<_>>()
}

fn solve_with_dist(input: &PreparedInput, dist: usize) -> usize {
    let columns = input.iter().map(|(_, i)| *i).collect::<FxHashSet<_>>();
    let rows = input.iter().map(|(j, _)| *j).collect::<FxHashSet<_>>();

    let increase_factor = dist - 1;

    let cumulative_gaps = |set: &FxHashSet<usize>| -> Vec<usize> {
        (0..set.iter().max().unwrap() + 1)
            .scan(0, |previous, i| {
                if !set.contains(&i) {
                    *previous += 1;
                }
                Some(*previous)
            })
            .collect::<Vec<_>>()
    };

    let expansion_x = cumulative_gaps(&columns);
    let expansion_y = cumulative_gaps(&rows);

    let expanded = input
        .iter()
        .map(|(j, i)| {
            (
                *j + expansion_y[*j] * increase_factor,
                *i + expansion_x[*i] * increase_factor,
            )
        })
        .collect::<FxHashSet<_>>();

    expanded
        .iter()
        .flat_map(|a| {
            expanded
                .iter()
                .filter_map(move |b| if a >= b { None } else { Some((a, b)) })
        })
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum()
}

pub fn solve_part1(input: &PreparedInput) -> usize {
    solve_with_dist(input, 2)
}

pub fn solve_part2(input: &PreparedInput) -> usize {
    solve_with_dist(input, 1_000_000)
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 9);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 374);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_with_dist(&prepare(EXAMPLE_INPUT), 10), 1030);
        assert_eq!(solve_with_dist(&prepare(EXAMPLE_INPUT), 100), 8410);
    }
}
