use winnow::prelude::*;
use winnow::stream::{Stream, StreamIsPartial};
use winnow::token::{one_of, take};
use winnow::PResult;

pub fn take_const<const N: usize, I, T>(input: &mut I) -> PResult<[T; N]>
where
    I: StreamIsPartial,
    I: Stream<Token = T>,
    I::Slice: TryInto<[T; N]>,
    <I::Slice as TryInto<[T; N]>>::Error: std::fmt::Debug,
{
    let tokens = take(N).parse_next(input)?;
    Ok(tokens.try_into().unwrap())
}

pub fn parse_single_u8<I>(input: &mut I) -> PResult<u8>
where
    I: StreamIsPartial,
    I: Stream<Token = u8>,
{
    one_of(b'0'..=b'9').map(|c| c - b'0').parse_next(input)
}
