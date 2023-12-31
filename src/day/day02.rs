use crate::solution::Solution;
use bstr::ByteSlice;
use winnow::ascii::dec_uint;
use winnow::combinator::{fail, separated, separated_foldl1, separated_pair};
use winnow::dispatch;
use winnow::prelude::*;
use winnow::stream::Accumulate;
use winnow::token::{any, take};

#[derive(Copy, Clone, Default)]
pub struct Draw {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Accumulate<(u8, Color)> for Draw {
    fn initial(_capacity: Option<usize>) -> Self {
        Draw::default()
    }

    fn accumulate(&mut self, acc: (u8, Color)) {
        match acc.1 {
            Color::Red => self.red += acc.0,
            Color::Green => self.green += acc.0,
            Color::Blue => self.blue += acc.0,
        }
    }
}

#[derive(Clone)]
pub struct Game {
    id: u8,
    largest_draw: Draw,
}

fn parse_draw(input: &mut &[u8]) -> PResult<Draw> {
    separated(
        1..,
        separated_pair(
            dec_uint,
            " ",
            dispatch! { any;
                b'r' => take(2usize).value(Color::Red),
                b'g' => take(4usize).value(Color::Green),
                b'b' => take(3usize).value(Color::Blue),
                _ => fail,
            },
        ),
        ", ",
    )
    .parse_next(input)
}
fn parse_game(input: &mut &[u8]) -> PResult<Game> {
    "Game ".parse_next(input)?;
    let id = dec_uint(input)?;
    ": ".parse_next(input)?;

    let largest_draw = separated_foldl1(parse_draw, "; ", |largest_draw, _, draw| Draw {
        red: largest_draw.red.max(draw.red),
        green: largest_draw.green.max(draw.green),
        blue: largest_draw.blue.max(draw.blue),
    })
    .parse_next(input)?;
    Ok(Game { id, largest_draw })
}

pub fn prepare(input: &str) -> impl Iterator<Item = Game> + '_ {
    input
        .as_bytes()
        .lines()
        .map(|line| parse_game.parse(line).unwrap())
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let games = prepare(input);

    let (part1, part2) = games.fold((0u32, 0u32), |(mut part1, mut part2), game| {
        if game.largest_draw.red <= 12
            && game.largest_draw.green <= 13
            && game.largest_draw.blue <= 14
        {
            part1 += game.id as u32;
        }
        part2 += game.largest_draw.red as u32
            * game.largest_draw.green as u32
            * game.largest_draw.blue as u32;

        (part1, part2)
    });

    (part1.into(), part2.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).count(), 5);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve(EXAMPLE_INPUT).0, 8u32.into());
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve(EXAMPLE_INPUT).1, 2286u32.into());
    }
}
