use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, alpha1, alphanumeric1, digit1, line_ending, newline, space1},
    combinator::rest,
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};
use std::collections::BTreeSet;
use std::iter;

#[derive(Debug)]
struct Race {
    record: u64,
    time: u64,
}

fn parse(input: &str) -> IResult<&str, Race> {
    let (input, times) = terminated(
        preceded(
            tag("Time:"),
            preceded(space1, separated_list1(space1, digit1)),
        ),
        newline,
    )(input)?;
    let (input, distances) = preceded(
        tag("Distance:"),
        preceded(space1, separated_list1(space1, digit1)),
    )(input)?;

    let time = times.join("").parse::<u64>().expect("parseable");
    let record = distances.join("").parse::<u64>().expect("parseable");

    Ok((input, Race { time, record }))
}

fn process(input: &str) -> u64 {
    let (input, race) = parse(input).expect("to be parsed");

    let mut possibilities = 0;

    for i in 0..race.time {
        let distance = i * (race.time - i);
        if distance > race.record {
            possibilities += 1;
        }
    }

    possibilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_example() {
        let input = r#"
Time:      7  15   30
Distance:  9  40  200
"#
        .trim();

        let result = process(input);

        assert_eq!(result, 71503);
    }

    #[test]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 74698);
    }
}
