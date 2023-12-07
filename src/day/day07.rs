use crate::solution::Solution;
use bstr::ByteSlice;
use winnow::ascii::dec_uint;
use winnow::Parser;

type Hand = u32;

type PreparedInput = Vec<(Hand, u16)>;

pub fn prepare(input: &str) -> PreparedInput {
    input
        .as_bytes()
        .lines()
        .map(|line| {
            let hand: Hand = line[0..5].iter().fold(0u32, |num, c| {
                (num << 4)
                    + match c {
                        b'2'..=b'9' => (c - b'0') as u32,
                        b'A' => 14,
                        b'K' => 13,
                        b'Q' => 12,
                        b'J' => 11,
                        b'T' => 10,
                        _ => panic!(),
                    }
            });

            let bid: u16 = dec_uint::<_, _, ()>.parse(&line[6..]).unwrap();
            (hand, bid)
        })
        .collect()
}

fn top_two<T, I>(list: I, min: T) -> [T; 2]
where
    T: Copy + Ord,
    I: IntoIterator<Item = T>,
{
    let mut best = [min, min];
    list.into_iter().for_each(|item| {
        if item > best[1] {
            if item > best[0] {
                best[1] = best[0];
                best[0] = item;
            } else {
                best[1] = item;
            }
        }
    });

    best
}

fn score_hand<const JOKER: bool>(hand: &Hand) -> u32 {
    let mut hand = *hand;

    let mut map = [0u8; 13];
    let mut joker_count = 0;
    for i in 0..5 {
        let card = hand >> ((4 - i) * 4) & 15;
        if JOKER && card == 11 {
            joker_count += 1;
            hand &= !(15 << ((4 - i) * 4));
        } else {
            map[card as usize - 2] += 1;
        }
    }

    let [mut a, b] = top_two(map, 0);
    if JOKER {
        a += joker_count;
    }

    ((a as u32) << 23) + ((b as u32) << 20) + hand
}

fn solve_joker<const JOKER: bool>(input: &PreparedInput) -> u64 {
    let mut scored: Vec<(u32, u16)> = input
        .iter()
        .map(|(hand, bid)| (score_hand::<JOKER>(hand), *bid))
        .collect();

    scored.sort_unstable_by_key(|(score, _)| *score);

    scored
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u64 * bid as u64)
        .sum()
}

pub fn solve_part1(input: &PreparedInput) -> u64 {
    solve_joker::<false>(input)
}

pub fn solve_part2(input: &PreparedInput) -> u64 {
    solve_joker::<true>(input)
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 5);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 6440);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 5905);
    }
}
