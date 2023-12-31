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
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
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

fn parse_card((cards, bid): (&str, u32)) -> CardHand {
    let n_j = cards.chars().filter(|c| c == &'J').count();

    let cards_value_ = cards.chars().filter(|&c| c != 'J').counts_by(|a| a);

    let cards_value = cards_value_
        .values()
        .sorted_by(|a, b| Ord::cmp(&a, &b))
        .rev()
        .enumerate();

    let mut hand_value = [n_j, 0, 0, 0, 0];

    for (i, v) in cards_value {
        hand_value[i] += v;
    }

    let hand_type = match hand_value[..] {
        [5, 0, 0, 0, 0] => HandType::Five,
        [4, 1, 0, 0, 0] => HandType::Four,
        [3, 2, 0, 0, 0] => HandType::FH,
        [3, 1, 1, 0, 0] => HandType::Three,
        [2, 2, 1, 0, 0] => HandType::TwoPairs,
        [2, 1, 1, 1, 0] => HandType::OnePair,
        _ => HandType::HighHand,
    };

    CardHand {
        cards,
        bid,
        hand_type,
    }
}

fn parse(input: &str) -> IResult<&str, Vec<CardHand>> {
    let (input, cards) = separated_list1(
        newline,
        map(
            separated_pair(alphanumeric1, space1, complete::u32),
            parse_card,
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

        assert_eq!(result, 5905);
    }

    #[test]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 253253225);
    }
}
