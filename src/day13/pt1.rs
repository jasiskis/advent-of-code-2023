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
    Ash,
    Rock,

    #[default]
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
enum Reflection {
    Row(u32),
    Column(u32),
}

fn find_row_reflection(grid: &Grid<Terrain>) -> Reflection {
    let result = grid
        .iter_rows()
        .enumerate()
        .tuple_windows()
        .filter(|pair| {
            let ((index_a, a), (index_b, b)) = pair;

            a.clone().collect::<Vec<_>>() == b.clone().collect::<Vec<_>>()
        })
        .find(|((a, _), (b, _))| {
            let mut index_a = *a as i32;
            let mut index_b = *b;

            let mut equal = true;
            loop {
                index_a -= 1;
                index_b += 1;
                if index_a < 0 || index_b >= grid.rows() {
                    break;
                }

                let row_a = grid.iter_row(index_a as usize);
                let row_b = grid.iter_row(index_b);

                if !row_a.eq(row_b) {
                    equal = false;
                    break;
                }
            }

            equal
        });

    match result {
        Some((_, (a, b))) => Reflection::Row(u32::try_from(a).unwrap()),
        _ => Reflection::Row(0),
    }
}

fn find_column_reflection(grid: &Grid<Terrain>) -> Reflection {
    let result = grid
        .iter_cols()
        .enumerate()
        .tuple_windows()
        .filter(|pair| {
            let ((index_a, a), (index_b, b)) = pair;

            a.clone().collect::<Vec<_>>() == b.clone().collect::<Vec<_>>()
        })
        .find(|((a, _), (b, _))| {
            let mut index_a = *a as i32;
            let mut index_b = *b;

            let mut equal = true;
            loop {
                index_a -= 1;
                index_b += 1;
                if index_a < 0 || index_b >= grid.cols() {
                    break;
                }

                let row_a = grid.iter_col(index_a as usize);
                let row_b = grid.iter_col(index_b);

                if !row_a.eq(row_b) {
                    equal = false;
                    break;
                }
            }

            equal
        });

    match result {
        Some((_, (a, b))) => Reflection::Column(u32::try_from(a).unwrap()),
        _ => Reflection::Column(0),
    }
}

fn find_reflection(grid: &Grid<Terrain>) -> Vec<Reflection> {
    let column = find_column_reflection(grid);
    let row = find_row_reflection(grid);

    vec![column, row]
}

fn parse(input: &str) -> Vec<Grid<Terrain>> {
    input
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => Terrain::Rock,
                            '.' => Terrain::Ash,
                            _ => panic!("ai ai ai"),
                        })
                        .collect()
                })
                .fold(Grid::new(0, 0), |mut acc, r| {
                    acc.push_row(r);
                    acc
                })
        })
        .collect()
}

fn process(input: &str) -> u32 {
    let grids = parse(input);

    grids
        .iter()
        .flat_map(find_reflection)
        .map(|r| match r {
            Reflection::Column(x) => x,
            Reflection::Row(x) => x * 100,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn simple_grid() {
        use Reflection::*;
        use Terrain::*;

        let input = grid![[Ash, Ash, Rock, Rock] [Ash, Ash, Rock, Rock] [Rock, Ash, Rock, Ash]];

        assert_eq!(find_reflection(&input), vec![Column(0), Row(1)]);
    }

    #[rstest]
    #[case("
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.".trim(), 5)]
    #[case("
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#".trim(), 400)]
    #[case("
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#".trim(),
        405)]
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
