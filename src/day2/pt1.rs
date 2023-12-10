fn process(input: &str) -> u32 {
    let lines = input.lines();

    let sum_valid_games = lines
        .map(|line| {
            let game: Vec<&str> = line.split(':').collect();

            match game[..] {
                [gameinfo, drafts] => {
                    let game_number: String = gameinfo.chars().filter(|c| c.is_numeric()).collect();
                    let drafts: Vec<&str> = drafts.split(';').collect();
                    let mut valid = true;

                    for draft in &drafts {
                        let parsed_drafts: Vec<Vec<&str>> = draft
                            .split(',')
                            .map(|x| x.trim().split(' ').collect())
                            .collect();

                        for d in parsed_drafts {
                            match d[..] {
                                [number, color] => {
                                    let number =
                                        number.parse::<u32>().expect("should be a number!");
                                    match color {
                                        "blue" if number > 14 => valid = false,
                                        "green" if number > 13 => valid = false,
                                        "red" if number > 12 => valid = false,
                                        _ => valid = true,
                                    }
                                }
                                _ => panic!("invalid input"),
                            }
                            if !valid {
                                break;
                            }
                        }

                        if !valid {
                            break;
                        }
                    }

                    if valid {
                        game_number.parse::<u32>().expect("should be a number!")
                    } else {
                        0
                    }
                }
                _ => panic!("invalid input"),
            }
        })
        .sum();

    return sum_valid_games;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_example() {
        let result = 2 + 2;

        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let result = process(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn real_input() {
        let result = 2 + 2;

        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 2600);
    }
}

