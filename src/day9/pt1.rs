use itertools::{FoldWhile, Itertools};
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

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse::<i64>().unwrap()).collect())
        .collect::<Vec<Vec<i64>>>()
}

fn process(input: &str) -> i64 {
    let vec = parse(input);

    let items: Vec<i64> = vec
        .iter()
        .map(|new_row| {
            let mut last_items: Vec<i64> = vec![];

            let mut current_row = new_row.clone();

            loop {
                let condition = current_row.iter().all(|x| x == &0);

                if condition {
                    break;
                } else {
                    current_row = current_row
                        .iter()
                        .tuple_windows()
                        .with_position()
                        .map(|(position, (left, right))| {
                            match position {
                                itertools::Position::Only | itertools::Position::Last => {
                                    last_items.push(*right)
                                }
                                _ => {}
                            }
                            right - left
                        })
                        .collect::<Vec<i64>>();
                }
            }

            last_items.iter().sum()
        })
        .collect();

    items.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        114
    )]
    fn base_example(#[case] input: &str, #[case] expected: i64) {
        let result = process(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 1479011877);
    }
}
