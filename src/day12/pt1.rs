use core::fmt;
use itertools::Itertools;
use std::collections::{BTreeSet, VecDeque};
use std::iter;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{
        self, alpha1, alphanumeric1, anychar, digit1, line_ending, newline, none_of, space1,
    },
    combinator::{map, rest},
    multi::{many0, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone)]
struct SpringRecord {
    springs: VecDeque<Spring>,
    damaged: VecDeque<i32>,
}

#[derive(Debug, Default, Clone)]
enum Spring {
    Damaged,
    Operational,
    #[default]
    Unkown,
}

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Spring::*;
        match self {
            Damaged => write!(f, "#"),
            Operational => write!(f, "."),
            Unkown => write!(f, "?"),
        }
    }
}

fn parse_springs(input: Vec<char>) -> VecDeque<Spring> {
    dbg!(&input);
    input
        .iter()
        .map(|c| match c {
            '#' => Spring::Damaged,
            '.' => Spring::Operational,
            '?' => Spring::Unkown,
            _ => panic!("shouldn't be here!"),
        })
        .collect()
}

fn parse(input: &str) -> IResult<&str, Vec<SpringRecord>> {
    let (input, records) = separated_list1(
        newline,
        map(
            separated_pair(
                many0(none_of(" ")),
                space1,
                separated_list1(tag(","), complete::i32),
            ),
            |(springs, damaged): (Vec<char>, Vec<i32>)| SpringRecord {
                springs: parse_springs(springs),
                damaged: VecDeque::from(damaged),
            },
        ),
    )(input)?;

    Ok((input, records))
}

fn compute_arrangement(
    mut springs: VecDeque<Spring>,
    mut remaining_damaged: VecDeque<i32>,
    damaged_count: i32,
    mut so_far: Vec<Spring>,
) -> i32 {
    let springs_l = &springs.clone();
    let rmd = &remaining_damaged.clone();
    let damaged_count_l = &damaged_count.clone();
    let log = format!(
        "{:?} {:?} {}",
        springs_l.iter().map(|x| x.to_string()).join(""),
        rmd,
        damaged_count_l
    );

    let log = format!(
        "{:?} --- {:?} {:?} {}",
        &so_far.iter().map(|x| x.to_string()).join(""),
        springs_l.iter().map(|x| x.to_string()).join(""),
        rmd,
        damaged_count_l
    );
    // println!("{}", &log);
    if remaining_damaged.is_empty() {
        println!("{}", &log);

        return 1;
    }

    let current_spring = springs.pop_front();

    let mut current_group = remaining_damaged.front().unwrap();

    let x = match current_spring {
        Some(Spring::Operational) => {
            so_far.push(Spring::Operational);
            if &damaged_count == current_group {
                remaining_damaged.pop_front();
                compute_arrangement(springs, remaining_damaged, 0, so_far)
            } else if damaged_count > 0 && *current_group > damaged_count {
                0
            } else {
                compute_arrangement(springs, remaining_damaged, damaged_count, so_far)
            }
        }
        Some(Spring::Damaged) => {
            so_far.push(Spring::Damaged);
            if (damaged_count + 1) > *current_group {
                0
            } else {
                compute_arrangement(springs, remaining_damaged, damaged_count + 1, so_far)
            }
        }
        Some(Spring::Unkown) => {
            let mut operational = springs.clone();

            let mut sfd = so_far.clone();
            sfd.push(Spring::Damaged);
            let mut sfo = so_far.clone();
            sfo.push(Spring::Operational);

            springs.push_front(Spring::Damaged);
            operational.push_front(Spring::Operational);

            compute_arrangement(springs, remaining_damaged.clone(), damaged_count, sfd)
                + compute_arrangement(operational, remaining_damaged.clone(), damaged_count, sfo)
        }
        None => {
            if *current_group == damaged_count {
                println!("{}", &log);
                return 1;
            } else {
                0
            }
        }
    };

    // println!("{} => {}", &log, x);
    x
}

fn arrangement(springs: VecDeque<Spring>, remaining_damaged: VecDeque<i32>) -> i32 {
    compute_arrangement(springs, remaining_damaged, 0, Vec::new())
}

fn process(input: &str) -> i32 {
    let (input, records) = parse(input).expect("xuxu");

    records
        .into_iter()
        .map(|record| arrangement(record.springs, record.damaged))
        .inspect(|x| {
            dbg!(x);
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn record1() {
        use Spring::*;
        let springs = VecDeque::from(vec![
            Unkown,
            Unkown,
            Unkown,
            Operational,
            Damaged,
            Damaged,
            Damaged,
        ]);
        let damaged = VecDeque::from(vec![1, 1, 3]);

        let result = arrangement(springs, damaged);
        assert_eq!(result, 1);
    }

    #[test]
    fn record2() {
        use Spring::*;
        let springs = VecDeque::from(vec![
            Operational,
            Unkown,
            Unkown,
            Operational,
            Operational,
            Unkown,
            Unkown,
            Operational,
            Operational,
            Operational,
            Unkown,
            Damaged,
            Damaged,
            Operational,
        ]);
        let damaged = VecDeque::from(vec![1, 1, 3]);

        let result = arrangement(springs, damaged);
        assert_eq!(result, 4);
    }

    #[test]
    fn record5() {
        use Spring::*;
        let springs = VecDeque::from(vec![
            Unkown,
            Unkown,
            Unkown,
            Unkown,
            Operational,
            Damaged,
            Damaged,
            Damaged,
            Damaged,
            Damaged,
            Damaged,
            Operational,
            Operational,
            Damaged,
            Damaged,
            Damaged,
            Damaged,
            Damaged,
            Operational,
        ]);
        let damaged = VecDeque::from(vec![1, 6, 5]);

        let result = arrangement(springs, damaged);
        assert_eq!(result, 4);
    }

    #[test]
    fn record6() {
        use Spring::*;
        let springs = VecDeque::from(vec![
            Unkown, Damaged, Damaged, Damaged, Unkown, Unkown, Unkown, Unkown, Unkown, Unkown,
            Unkown, Unkown,
        ]);
        let damaged = VecDeque::from(vec![3, 2, 1]);

        let result = arrangement(springs, damaged);
        assert_eq!(result, 10);
    }

    #[rstest]
    #[ignore]
    #[case("
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
".trim(), 21)]
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
