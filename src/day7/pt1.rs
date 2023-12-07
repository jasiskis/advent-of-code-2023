use itertools::Itertools;
use std::iter;
use std::{cmp::Ordering, collections::BTreeMap};

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

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    Five = 6,
    Four = 5,
    FH = 4,
    Three = 3,
    TwoPairs = 2,
    OnePair = 1,
    HighHand = 0,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct CardHand<'a> {
    cards: &'a str,
    bid: u32,
    hand_type: HandType,
}

impl<'a> Ord for CardHand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self
                .cards
                .chars()
                .zip(other.cards.chars())
                .find(|(a, b)| a != b)
                .map(|(a, b)| CARD_POWER[&a].cmp(&CARD_POWER[&b]))
                .unwrap(),
            x => x,
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<CardHand>> {
    let (input, cards) = separated_list1(
        newline,
        map(
            separated_pair(alphanumeric1, space1, complete::u32),
            |(cards, bid): (&str, u32)| {
                let hand_type = match cards
                    .chars()
                    .counts_by(|a| a)
                    .values()
                    .sorted_by(|a, b| Ord::cmp(&a, &b))
                    .rev()
                    .collect::<Vec<&usize>>()[..]
                {
                    [5] => HandType::Five,
                    [4, 1] => HandType::Four,
                    [3, 2] => HandType::FH,
                    [3, 1, 1] => HandType::Three,
                    [2, 2, 1] => HandType::TwoPairs,
                    [2, 1, 1, 1] => HandType::OnePair,
                    _ => HandType::HighHand,
                };

                CardHand {
                    cards,
                    bid,
                    hand_type,
                }
            },
        ),
    )(input)?;

    Ok((input, cards))
}

fn process(input: &str) -> u32 {
    let (input, result) = parse(input).expect("stuff");

    let x = result
        .iter()
        .sorted_by(|a, b| a.cmp(b))
        .enumerate()
        .inspect(|x| println!("{:?}", &x))
        .map(|(i, val)| val.bid * ((i + 1) as u32))
        .sum::<u32>();

    dbg!(&result);

    x
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

        assert_eq!(result, 6440);
    }

    #[test]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 253638586);
    }
}
