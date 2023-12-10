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

#[derive(Debug, PartialEq, Eq, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Start,
    Empty,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::Start,
            _ => Self::Empty,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .map(|line| line.chars().map(Pipe::from).collect())
        .collect()
}

#[derive(Debug)]
struct Move {
    row: i32,
    column: i32,
    allowed_pipes: Vec<Pipe>,
}

fn possible_moves(pipe: &Pipe) -> Vec<Move> {
    match pipe {
        Pipe::Vertical => vec![
            Move {
                row: -1,
                column: 0,
                allowed_pipes: vec![Pipe::Vertical, Pipe::SE, Pipe::SW],
            },
            Move {
                row: 1,
                column: 0,
                allowed_pipes: vec![Pipe::Vertical, Pipe::NE, Pipe::NW],
            },
        ],
        Pipe::Horizontal => vec![
            Move {
                row: 0,
                column: -1,
                allowed_pipes: vec![Pipe::Horizontal, Pipe::NE, Pipe::NW, Pipe::SE, Pipe::SW],
            },
            Move {
                row: 0,
                column: 1,
                allowed_pipes: vec![Pipe::Horizontal, Pipe::NE, Pipe::NW, Pipe::SE, Pipe::SW],
            },
        ],
        Pipe::NE => vec![
            Move {
                row: -1,
                column: 0,
                allowed_pipes: vec![Pipe::Vertical, Pipe::SE, Pipe::SW],
            },
            Move {
                row: 0,
                column: 1,
                allowed_pipes: vec![Pipe::Horizontal, Pipe::NW, Pipe::SW],
            },
        ],

        // J
        Pipe::NW => vec![
            Move {
                row: -1,
                column: 0,
                allowed_pipes: vec![Pipe::Vertical, Pipe::SE, Pipe::SW],
            },
            Move {
                row: 0,
                column: -1,
                allowed_pipes: vec![Pipe::Horizontal, Pipe::NE, Pipe::SE],
            },
        ],

        // F
        Pipe::SE => vec![
            Move {
                row: 1,
                column: 0,
                allowed_pipes: vec![Pipe::Vertical, Pipe::NE, Pipe::NW],
            },
            Move {
                row: 0,
                column: 1,
                allowed_pipes: vec![Pipe::Horizontal, Pipe::NW, Pipe::SW],
            },
        ],

        Pipe::SW => vec![
            Move {
                row: 1,
                column: 0,
                allowed_pipes: vec![Pipe::Vertical, Pipe::NE, Pipe::NW],
            },
            Move {
                row: 0,
                column: -1,
                allowed_pipes: vec![Pipe::Horizontal, Pipe::NE, Pipe::SE],
            },
        ],
        Pipe::Start => vec![
            Move {
                row: -1,
                column: 0,
                allowed_pipes: vec![Pipe::Vertical, Pipe::SE, Pipe::SW],
            },
            Move {
                row: 1,
                column: 0,
                allowed_pipes: vec![Pipe::Vertical, Pipe::NE, Pipe::NW],
            },
            Move {
                row: 0,
                column: -1,
                allowed_pipes: vec![Pipe::Horizontal, Pipe::NE, Pipe::NW, Pipe::SE, Pipe::SW],
            },
            Move {
                row: 0,
                column: 1,
                allowed_pipes: vec![Pipe::Horizontal, Pipe::NE, Pipe::NW, Pipe::SE, Pipe::SW],
            },
        ],
        _ => vec![],
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    row: usize,
    column: usize,
    pipe: Pipe,
}

fn process(input: &str) -> u32 {
    let grid = parse(input);

    let mut pipes = grid
        .iter()
        .enumerate()
        .find_map(|(row, x)| {
            x.iter()
                .enumerate()
                .find(|(_, y)| y == &&Pipe::Start)
                .map(|(column, _)| Position {
                    row,
                    column,
                    pipe: Pipe::Start,
                })
        })
        .into_iter()
        .collect::<Vec<Position>>();

    let mut previous_pipe = Position {
        row: 0,
        column: 0,
        pipe: Pipe::Empty,
    };

    loop {
        let current_position = pipes.last().unwrap();

        let next_moves = possible_moves(&current_position.pipe);

        for m in next_moves {
            let new_row = i32::try_from(current_position.row).unwrap() + m.row;
            let new_column = i32::try_from(current_position.column).unwrap() + m.column;

            let next_pipe_opt = match (usize::try_from(new_row), usize::try_from(new_column)) {
                (Ok(r), Ok(c)) => grid.get(r).and_then(|v| {
                    v.get(c).map(|p| Position {
                        row: r,
                        column: c,
                        pipe: p.clone(),
                    })
                }),
                _ => None,
            };

            match next_pipe_opt {
                Some(next_pipe) => {
                    if (m.allowed_pipes.contains(&next_pipe.pipe) || next_pipe.pipe == Pipe::Start)
                        && next_pipe != previous_pipe
                    {
                        previous_pipe = current_position.clone();
                        pipes.push(next_pipe.clone());

                        break;
                    }
                }
                None => continue,
            }
        }

        if pipes.last().is_some_and(|p| p.pipe == Pipe::Start) {
            break;
        }
    }

    u32::try_from((pipes.len() - 1) / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        4
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        8
    )]
    fn base_example(#[case] input: &str, #[case] expected: u32) {
        let result = process(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 6800);
    }
}
