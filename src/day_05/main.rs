use anyhow::{Result, anyhow};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use crate::parse::{Mapping};

#[macro_use]
extern crate simple_log;

mod parse {
    use std::collections::HashMap;
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use std::ops::Range;

    #[derive(Debug)]
    pub struct Input {
        pub seeds: Vec<u64>,
        pub seed_ranges: Vec<Range<u64>>,
        pub mappings: Vec<Mapping>
    }

    impl Input {
        pub fn new(seeds: Vec<u64>, mappings: Vec<Mapping>) -> Self {
            assert!(seeds.len() % 2 == 0);

            let seed_ranges = seeds.chunks(2)
                .map(|chunk| {
                    let start = *chunk.get(0).expect("slice of size 2");
                    let length = *chunk.get(1).expect("slice of size 2");
                    start..(start + length)
                })
                .collect();


            Input {
                seeds,
                seed_ranges,
                mappings
            }
        }

    }

    #[derive(Debug)]
    pub struct Mapping {
        pub name: String,
        pub ranges: Vec<MappingRange>,
        pub map_cache: HashMap<u64, u64>
    }

    #[derive(Debug)]
    pub struct MappingRange {
        pub source: Range<u64>,
        pub dest: Range<u64>
    }

    impl MappingRange {
        fn new(source_start: u64, dest_start: u64, length: u64) -> Self {
            MappingRange {
                source: source_start..(source_start + length),
                dest: dest_start..(dest_start + length),
            }
        }
    }

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(
            line("seeds: " repeat_sep(u64, " "))
            line("")
            sections(
                line(string(any_char+) " map:")
                lines(repeat_sep(u64, " "))
            )
        );

        let raw_data = read_to_string(filename)?;
        let raw_parsed = parser.parse(&raw_data).context("parse error")?;
        let (
            seeds,
            _,
            raw_maps
        ) = raw_parsed;

        let mappings: Vec<Mapping> = raw_maps
            .into_iter()
            .map(|(name, raw_ranges)| {
                let ranges: Vec<MappingRange> = raw_ranges.into_iter()
                    .map(|raw_range: Vec<u64>| {
                        let dest_start: u64 = *raw_range.get(0).expect("element at 0");
                        let source_start: u64 = *raw_range.get(1).expect("element at 1");
                        let length: u64 = *raw_range.get(2).expect("element at 2");
                        MappingRange::new(source_start, dest_start, length)
                    })
                    .collect();
                Mapping { name, ranges, map_cache: HashMap::new() }
            })
            .collect();

        Ok(Input::new(seeds, mappings))
    }
}

impl Mapping {
    pub fn map(self: &mut Self, input: u64) -> u64 {
        // I tried memoization here, but that blows up for part 2 since it uses >15GB memory
        for range in self.ranges.iter() {
            if range.source.contains(&input) {
                let offset = input - range.source.start;
                return range.dest.start + offset
            }
        }

        // no mapping found, input maps to output
        input
    }
}

fn solve_part_1(filename: &str) -> Result<u64> {
    // need mutability b/c of internal caching
    let mut input = parse::parse_input(filename)?;

    let mut lowest: Option<u64> = None;
    for seed in input.seeds {
        let mut current_number = seed;
        for mapping in input.mappings.iter_mut() {
            let result = mapping.map(current_number);
            current_number = result
        }
        lowest.replace(u64::min(current_number, lowest.unwrap_or(current_number)));
    }

    lowest.ok_or(anyhow!("No lowest number found"))
}

// TODO: this currently takes 4,5 minutes, maybe we can make it faster
fn solve_part_2(filename: &str) -> Result<u64> {
    // need mutability b/c of internal caching
    let mut input = parse::parse_input(filename)?;

    let total: u64 = input.seed_ranges.clone().into_iter()
        .map(|range| range.try_len().expect("size hint") as u64)
        .sum();
    let bar = ProgressBar::new(total);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta_precise}] {bar:80.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

    let mut lowest: Option<u64> = None;
    let seed_iters = input.seed_ranges.into_iter()
        .flat_map(|range| range.into_iter());
    for seed in seed_iters.into_iter() {
        let mut current_number = seed;
        for mapping in input.mappings.iter_mut() {
            let result = mapping.map(current_number);
            current_number = result
        }
        lowest.replace(u64::min(current_number, lowest.unwrap_or(current_number)));
        bar.inc(1)
    }

    bar.finish();
    lowest.ok_or(anyhow!("No lowest number found"))
}

fn main() -> Result<()> {
    simple_log::quick!("info");
    info!("Result part 1: {}", solve_part_1("src/day_05/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_05/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{parse, solve_part_1, solve_part_2};
    use crate::parse::Mapping;

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_05/test_input.txt").unwrap();
        assert_eq!(result, 35);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_05/test_input.txt").unwrap();
        assert_eq!(result, 46);
    }

    #[test]
    fn validate_range_mappings() {
        let mut input = parse::parse_input("src/day_05/test_input.txt")
            .expect("valid input");
        let seed_to_soil: &mut Mapping = input.mappings.get_mut(0).unwrap();

        assert_eq!(seed_to_soil.map(0), 0);
        assert_eq!(seed_to_soil.map(1), 1);
        assert_eq!(seed_to_soil.map(48), 48);
        assert_eq!(seed_to_soil.map(49), 49);
        assert_eq!(seed_to_soil.map(50), 52);
        assert_eq!(seed_to_soil.map(51), 53);
        assert_eq!(seed_to_soil.map(96), 98);
        assert_eq!(seed_to_soil.map(97), 99);
        assert_eq!(seed_to_soil.map(98), 50);
        assert_eq!(seed_to_soil.map(99), 51);

        assert_eq!(seed_to_soil.map(79), 81);
        assert_eq!(seed_to_soil.map(14), 14);
        assert_eq!(seed_to_soil.map(55), 57);
        assert_eq!(seed_to_soil.map(13), 13);
    }
}