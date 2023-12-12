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
                        Universe::Galaxy(Position { x: ii, y: i })
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

fn manhattan_distance(p1: &Position, p2: &Position, empty_x: &[i32], empty_y: &[i32]) -> i32 {
    let p1_plus_x = empty_x.get(p1.x).unwrap();
    let p1_plus_y = empty_y.get(p1.y).unwrap();

    let p2_plus_x = empty_x.get(p2.x).unwrap();
    let p2_plus_y = empty_y.get(p2.y).unwrap();
    match (
        i32::try_from(p1.x),
        i32::try_from(p1.y),
        i32::try_from(p2.x),
        i32::try_from(p2.y),
    ) {
        (Ok(p1x), Ok(p1y), Ok(p2x), Ok(p2y)) => {
            ((p1x + p1_plus_x) - (p2x + p2_plus_x)).abs()
                + ((p1y + p1_plus_y) - (p2y + p2_plus_y)).abs()
        }
        _ => 0,
    }
}

fn compute_empty(grid: &Grid<Universe>) -> (Vec<i32>, Vec<i32>) {
    let mut x_empty = vec![0; grid.cols()];
    let mut y_empty = vec![0; grid.rows()];

    let mut x_empty_acc = 0;
    for (index, mut col) in grid.iter_cols().enumerate() {
        if col.all(|x| matches!(x, Universe::Emptiness)) {
            x_empty_acc += 1;
        }
        x_empty[index] = x_empty_acc;
    }

    let mut y_empty_acc = 0;
    for (index, mut row) in grid.iter_rows().enumerate() {
        if row.all(|x| matches!(x, Universe::Emptiness)) {
            y_empty_acc += 1;
        }
        y_empty[index] = y_empty_acc;
    }

    (x_empty, y_empty)
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

    let (empty_x, empty_y) = compute_empty(&grid);

    let distances: Vec<i32> = x
        .iter()
        .map(|(x, y)| match (x, y) {
            (Universe::Galaxy(p1), Universe::Galaxy(p2)) => {
                manhattan_distance(p1, p2, &empty_x, &empty_y)
            }
            _ => 0,
        })
        .collect();

    distances.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
".trim(), 374)]
    #[case("", 0)]
    fn base_example(#[case] input: &str, #[case] expected: i32) {
        let result = process(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 10173804);
    }
}
