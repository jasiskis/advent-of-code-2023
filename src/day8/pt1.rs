use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};
use std::iter;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::{
        self,
        complete::{
            self, alpha1, alphanumeric1, char, digit1, line_ending, multispace1, newline, space1,
        },
    },
    combinator::rest,
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map<'a> {
    directions: Vec<Direction>,
    network: BTreeMap<&'a str, (&'a str, &'a str)>,
}

fn parse(input: &str) -> IResult<&str, Map> {
    let (input, directions) = many1(alt((
        char('L').map(|_| Direction::Left),
        char('R').map(|_| Direction::Right),
    )))(input)?;

    let (input, _) = multispace1(input)?;

    let (input, list) = separated_list1(
        newline,
        separated_pair(
            alphanumeric1,
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                tag(")"),
            ),
        ),
    )(input)?;

    let mut network = BTreeMap::new();
    for (key, value) in list {
        network.insert(key, value);
    }

    Ok((
        input,
        Map {
            directions,
            network,
        },
    ))
}

fn process(input: &str) -> u32 {
    let (input, map) = parse(input).expect("no errors");

    let (n, node) = map
        .directions
        .iter()
        .cycle()
        .fold_while((0, "AAA"), |(i, next_item), direction| {
            let (left, right) = map.network.get(next_item).unwrap();

            let after = match direction {
                Direction::Left => left,
                Direction::Right => right,
            };
            if after == &"ZZZ" {
                Done((i + 1, after))
            } else {
                Continue((i + 1, after))
            }
        })
        .into_inner();

    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        2
    )]
    #[case(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        6
    )]
    fn base_example(#[case] input: &str, #[case] expected: u32) {
        let result = process(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 11567);
    }
}
