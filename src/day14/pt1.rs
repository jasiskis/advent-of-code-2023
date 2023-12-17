use grid::{grid, Grid};
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

#[derive(Debug, Default, PartialEq, Eq)]
enum Terrain {
    Round,
    Cube,

    #[default]
    Empty,
}

fn parse(input: &str) -> Grid<Terrain> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Terrain::Cube,
                    'O' => Terrain::Round,
                    '.' => Terrain::Empty,
                    _ => panic!("ai ai ai"),
                })
                .collect()
        })
        .fold(Grid::new(0, 0), |mut acc, r| {
            acc.push_row(r);
            acc
        })
}

fn process(input: &str) -> u32 {
    let grids = parse(input);

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".trim(), 136)]
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
