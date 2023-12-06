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
    record: u32,
    time: u32,
}

fn parse(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = terminated(
        preceded(
            tag("Time:"),
            preceded(space1, separated_list1(space1, complete::u32)),
        ),
        newline,
    )(input)?;
    let (input, distances) = preceded(
        tag("Distance:"),
        preceded(space1, separated_list1(space1, complete::u32)),
    )(input)?;

    let vec = times
        .into_iter()
        .zip_eq(distances)
        .map(|(time, record)| Race { record, time })
        .collect();

    Ok((input, vec))
}

fn process(input: &str) -> u32 {
    let (input, races) = parse(input).expect("to be parsed");

    let mut results: Vec<u32> = Vec::new();
    for race in races {
        let mut possibilities = 0;

        for i in 0..race.time {
            let distance = i * (race.time - i);
            if distance > race.record {
                possibilities += 1;
            }
        }
        results.push(possibilities);
    }

    results.iter().fold(1, |acc, e| acc * e)
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

        assert_eq!(result, 288);
    }

    #[test]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 74698);
    }
}
