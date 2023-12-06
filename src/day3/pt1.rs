use itertools::Itertools;
use std::collections::BTreeSet;
use std::iter;

#[derive(Debug)]
struct PartNumber {
    start: usize,
    end: usize,
    number: u32,
}

fn process(input: &str) -> u32 {
    let lines = input.lines();

    let start = iter::once("");
    let end = iter::once("");
    let lines = start.chain(lines).chain(end);

    let tuples = lines.tuple_windows::<(_, _, _)>();
    let mut sum_part_numbers = 0;

    for tuple in tuples {
        let (line1, current_line, line3) = tuple;

        let symbols = line1
            .char_indices()
            .filter(|(i, c)| c != &'.' && !c.is_numeric());
        let symbols2 = current_line
            .char_indices()
            .filter(|(i, c)| c != &'.' && !c.is_numeric());
        let symbols3 = line3
            .char_indices()
            .filter(|(i, c)| c != &'.' && !c.is_numeric());

        let symbols_index = symbols
            .chain(symbols2)
            .chain(symbols3)
            .map(|(i, c)| i)
            .collect::<BTreeSet<usize>>();

        let mut temp_number = "".to_string();
        let mut start_index = 0;
        let mut it = current_line.char_indices().peekable();
        let mut part_numbers: Vec<PartNumber> = vec![];

        while let Some((i, c)) = it.next() {
            if c.is_numeric() {
                temp_number = temp_number + &c.to_string();
                if start_index == 0 {
                    start_index = i;
                }

                match it.peek() {
                    None if start_index != 0 => {
                        part_numbers.push(PartNumber {
                            start: start_index,
                            end: i,
                            number: temp_number.parse::<u32>().expect("should be a number!"),
                        });

                        start_index = 0;
                        temp_number = "".to_string();
                    }
                    Some((_, next)) if !next.is_numeric() => {
                        part_numbers.push(PartNumber {
                            start: start_index,
                            end: i,
                            number: temp_number.parse::<u32>().expect("should be a number!"),
                        });

                        start_index = 0;
                        temp_number = "".to_string();
                    }
                    _ => (),
                }
            }
        }

        let filtered_parts: Vec<_> = part_numbers
            .into_iter()
            .filter(|part_number| {
                (part_number.start - 1..=part_number.end + 1).any(|i| symbols_index.contains(&i))
            })
            .collect();

        sum_part_numbers += filtered_parts
            .iter()
            .map(|part_number| part_number.number)
            .sum::<u32>();
    }
    sum_part_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_example() {
        let result = 2 + 2;

        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let result = process(input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn another_example() {
        let result = 2 + 2;

        let input = r#".......5......
..7*..*.....4*
...*13*......9
.......15.....
..............
..............
..............
..............
..............
..............
21............
...*9.........
"#;

        let result = process(input);

        assert_eq!(result, 62);
    }

    #[test]
    fn real_input() {
        let result = 2 + 2;

        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 520019);
    }
}
