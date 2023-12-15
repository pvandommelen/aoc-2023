use crate::solution::Solution;

type PreparedInput<'a> = Vec<&'a [u8]>;

pub fn prepare(input: &str) -> PreparedInput {
    input.split(',').map(|line| line.as_bytes()).collect()
}

fn hash(bytes: &[u8]) -> u32 {
    bytes.iter().fold(0, |mut current, c| {
        current += *c as u32;
        current *= 17;
        current %= 256;
        current
    })
}

pub fn solve_part1(input: &PreparedInput) -> u32 {
    input.iter().map(|steps| hash(steps)).sum()
}

pub fn solve_part2(input: &PreparedInput) -> usize {
    let mut boxes: Vec<Vec<(&[u8], u8)>> = vec![vec![]; 256];

    input.iter().for_each(|steps| {
        if steps[steps.len() - 1] == b'-' {
            let label = &steps[..steps.len() - 1];
            let hash = hash(label) as usize;
            boxes[hash].retain(|entry| entry.0 != label);
        } else {
            let amount = steps[steps.len() - 1] - b'0';
            let label = &steps[..steps.len() - 2];
            let hash = hash(label) as usize;

            match boxes[hash].iter_mut().find(|entry| entry.0 == label) {
                None => boxes[hash].push((label, amount)),
                Some(entry) => entry.1 = amount,
            };
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_number, boxx)| {
            boxx.iter()
                .enumerate()
                .map(|(slot_number, entry)| (1 + box_number) * (slot_number + 1) * entry.1 as usize)
                .sum::<usize>()
        })
        .sum()
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 11);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 1320);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 145);
    }
}
