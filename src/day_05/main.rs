use anyhow::{Result, anyhow};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;
use crate::parse::{Mapping};

#[macro_use]
extern crate simple_log;

mod parse {
    use std::cmp::Ordering;
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
    }

    impl Mapping {
        fn new(name: String, ranges: Vec<MappingRange>) -> Self {
            Mapping {
                name,
                ranges,
            }
        }

        pub fn map(self: &Self, input: u64) -> u64 {
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

    #[derive(Debug, PartialEq, Eq)]
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
                Mapping::new(name, ranges)
            })
            .collect();

        Ok(Input::new(seeds, mappings))
    }
}

fn solve_part_1(filename: &str) -> Result<u64> {
    // need mutability b/c of internal caching
    let input = parse::parse_input(filename)?;

    let mut lowest: Option<u64> = None;
    for seed in input.seeds {
        let mut current_number = seed;
        for mapping in input.mappings.iter() {
            let result = mapping.map(current_number);
            current_number = result
        }
        lowest.replace(u64::min(current_number, lowest.unwrap_or(current_number)));
    }

    lowest.ok_or(anyhow!("No lowest number found"))
}

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

    let iter = input.seed_ranges.par_iter()
        .flat_map_iter(|range| range.clone().into_iter());

    let lowest = iter.fold(|| None, |lowest, seed: u64 | {
        let mut current_number = seed.clone();
        for mapping in input.mappings.iter() {
            let result = mapping.map(current_number);
            current_number = result
        }
        bar.inc(1);
        match lowest {
            None => Some(current_number),
            Some(lowest) => Some(u64::min(lowest, current_number))
        }
    }).reduce(|| None, |lowest_a, lowest_b| {
        if lowest_a.is_some() && lowest_b.is_some() {
            Some(u64::min(lowest_a.unwrap(), lowest_b.unwrap()))
        } else {
            lowest_a.or(lowest_b)
        }
    });

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