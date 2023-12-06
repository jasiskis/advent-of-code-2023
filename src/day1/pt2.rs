use itertools::Itertools;
use std::collections::BTreeSet;
use std::iter;

fn process(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_example() {
        let input = r#"
"#;

        let result = process(input);

        assert_eq!(result, 0);
    }

    #[test]
    fn real_input() {
        let input = include_str!("./input.txt");

        let result = process(input);

        assert_eq!(result, 0);
    }
}
