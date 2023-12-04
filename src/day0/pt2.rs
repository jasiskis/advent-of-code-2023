use std::collections::BTreeSet;
use std::iter;
use itertools::Itertools;

fn process(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_example() {
        let result = 2 + 2;

        let input = r#"
"#;


        let result = process(input);

        assert_eq!(result, 0);
    }

    #[test]
    fn real_input() {
        let result = 2 + 2;

        let input = include_str!("./input.txt");


        let result = process(input);

        assert_eq!(result, 0);
    }
}
