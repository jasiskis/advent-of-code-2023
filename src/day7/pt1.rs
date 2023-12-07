use itertools::Itertools;
use std::collections::BTreeMap;
use std::iter;

use lazy_static::lazy_static;
use nom::{
    bytes::complete::take_until,
    character::complete::{self, alpha1, alphanumeric1, digit1, line_ending, newline, space1},
    combinator::{map, rest},
    multi::{separated_list0, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated},
    IResult, Parser,
};

lazy_static! {
    static ref CARD_POWER: BTreeMap<char, u32> = BTreeMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);
}

struct CardHand<'a> {
    cards: &'a str,
    bid: u32,
}

// fn parse(input: &str) -> IResult<&str, Vec<CardHand>> {
//     let parser = separated_pair::<&str, &str, _, &str, _>(alphanumeric1, space1, digit1)
//         .map(|(cards, bid)| CardHand { cards, bid: 0 });
//
//     let (input, cards) = separated_list1(newline, parser).parse(input)?;
//
//     Ok((input, cards))
// }

fn parse(input: &str) -> IResult<&str, Vec<CardHand>> {
    let (input, cards) = separated_list1(
        newline,
        map(
            separated_pair(alphanumeric1, space1, complete::u32),
            |(cards, bid): (&str, u32)| CardHand { cards, bid },
        ),
    )(input)?;

    Ok((input, cards))
}

fn process(input: &str) -> u32 {
    let (input, result) = parse(input).expect("stuff");

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_example() {
        let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#
        .trim();

        let result = process(input);

        assert_eq!(result, 0);
    }

    #[test]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 0);
    }
}
