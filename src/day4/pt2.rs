use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending, space1},
    combinator::cut,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Card<'a> {
    card_id: &'a str,
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

fn parse_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
    let (input, numbers) = separated_list1(space1, complete::u32)(input)?;

    let mut n = HashSet::new();
    for num in numbers {
        n.insert(num);
    }
    Ok((input, n))
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, card_id) = preceded(tag("Card"), preceded(space1, digit1))(input)?;
    let (input, (winning_numbers, card_numbers)) = preceded(
        preceded(tag(":"), space1),
        separated_pair(
            parse_numbers,
            preceded(space1, preceded(tag("|"), space1)),
            parse_numbers,
        ),
    )(input)?;

    let card = Card {
        card_id,
        winning_numbers,
        card_numbers,
    };

    Ok((input, card))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = cut(separated_list1(line_ending, card))(input)?;

    Ok((input, cards))
}

fn process(input: &str) -> u32 {
    let (input, cards) = parse_cards(input).expect("should parse");

    let matching_numbers: u32 = cards
        .iter()
        .map(|c| c.card_numbers.intersection(&c.winning_numbers).count())
        .filter(|c| c > &0_usize)
        .map(|c| (0..c - 1).fold(1, |x, y| x * 2))
        .sum();

    matching_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_example() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#
        .trim();

        let result = process(input);

        assert_eq!(result, 30);
    }

    #[test]
    // #[ignore]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 0);
    }
}
