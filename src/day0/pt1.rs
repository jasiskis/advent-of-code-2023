use itertools::Itertools;
use std::collections::BTreeSet;
use std::iter;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, alpha1, alphanumeric1, digit1, line_ending, newline, space1},
    combinator::rest,
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

fn parse(input: &str) -> IResult<&str, u32> {
    Ok((input, 0))
}

fn process(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("", 0)]
    #[case("", 0)]
    fn base_example(#[case] input: &str, #[case] expected: u32) {
        let result = process(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 0);
    }
}
