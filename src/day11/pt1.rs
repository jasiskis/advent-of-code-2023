use itertools::Itertools;
use std::collections::{BTreeSet, HashSet};
use std::iter;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, alpha1, alphanumeric1, digit1, line_ending, newline, space1},
    combinator::rest,
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

use grid::*;

#[derive(Debug, PartialEq, Eq, Default, Clone, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Hash)]
enum Universe {
    Galaxy(Position),

    #[default]
    Emptiness,
}

fn parse(input: &str) -> Grid<Universe> {
    let grid: Vec<Vec<Universe>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(ii, c)| {
                    if c == '#' {
                        Universe::Galaxy(Position { x: i, y: ii })
                    } else {
                        Universe::Emptiness
                    }
                })
                .collect()
        })
        .collect();

    grid.iter().fold(Grid::<Universe>::new(0, 0), |mut acc, r| {
        acc.push_row(r.to_vec());
        acc
    })
}

fn manhattan_distance(p1: &Position, p2: &Position) -> i32 {
    dbg!(&p1, &p2);
    match (
        i32::try_from(p1.x),
        i32::try_from(p1.y),
        i32::try_from(p2.x),
        i32::try_from(p2.y),
    ) {
        (Ok(p1x), Ok(p1y), Ok(p2x), Ok(p2y)) => (p1x - p2x).abs() + (p1y - p2y).abs(),
        _ => 0,
    }
}

fn process(input: &str) -> i32 {
    let grid = parse(input);

    let galaxies: Vec<_> = grid
        .iter_rows()
        .flat_map(|x| {
            x.filter(|y| matches!(y, Universe::Galaxy(p)))
                .collect::<Vec<_>>()
        })
        .collect();

    let x: Vec<(&Universe, &Universe)> = galaxies.iter().cloned().tuple_combinations().collect();

    let distances: Vec<i32> = x
        .iter()
        .map(|(x, y)| match (x, y) {
            (Universe::Galaxy(p1), Universe::Galaxy(p2)) => manhattan_distance(p1, p2),
            _ => 0,
        })
        .collect();

    dbg!(&x.len());
    dbg!(&galaxies.len());

    distances.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
".trim(), 374)]
    #[case("", 0)]
    fn base_example(#[case] input: &str, #[case] expected: i32) {
        let result = process(input);

        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 0);
    }
}
