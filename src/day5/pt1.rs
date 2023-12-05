use itertools::Itertools;
use std::collections::{BTreeSet, VecDeque};
use std::iter;

#[derive(Debug)]
struct Coordinate {
    offset: u32,
    source_start: u32,
    dest_start: u32,
}

impl Coordinate {
    fn source_end(&self) -> u32 {
        self.source_start + self.offset
    }
    fn within_range(&self, seed: &u32) -> bool {
        seed >= &self.source_start && seed <= &self.source_end()
    }

    fn next_location(&self, seed: &u32) -> Option<u32> {
        if self.within_range(seed) {
            Some(self.dest_start + (seed - self.source_start))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map<'a> {
    source: &'a str,
    destination: &'a str,
    coordinates: Vec<Coordinate>,
}

impl<'a> Map<'a> {
    fn next_location(&self, source_position: u32) -> u32 {
        let coordinate = self
            .coordinates
            .iter()
            .find_map(|p| p.next_location(&source_position));

        coordinate.unwrap_or(source_position)
    }
}

struct Almanac<'a> {
    maps: Vec<Map<'a>>,
}

impl<'a> Almanac<'a> {
    fn find_location(&self, seed: u32) -> u32 {
        self.maps
            .iter()
            .fold(seed, |last_location, map| map.next_location(last_location))
    }
}

fn process(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_location() {
        let coordinate1 = Coordinate {
            offset: 2,
            dest_start: 50,
            source_start: 98,
        };

        assert_eq!(coordinate1.next_location(&78_u32), None);
        assert_eq!(coordinate1.next_location(&99_u32), Some(51));
    }

    #[test]
    fn map_next_location() {
        let coordinate1 = Coordinate {
            offset: 2,
            dest_start: 50,
            source_start: 98,
        };
        let coordinate2 = Coordinate {
            offset: 48,
            dest_start: 52,
            source_start: 50,
        };

        let map = Map {
            source: "seed",
            destination: "soil",
            coordinates: vec![coordinate1, coordinate2],
        };

        assert_eq!(map.next_location(33), 33);
        assert_eq!(map.next_location(53), 55);
        assert_eq!(map.next_location(99), 51);
    }

    #[test]
    fn base_example() {
        let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
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
