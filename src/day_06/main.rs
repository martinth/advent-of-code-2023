use std::ops::{RangeInclusive};
use anyhow::{Result};
use crate::parse::Race;

#[macro_use]
extern crate simple_log;

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;

    #[derive(Debug)]
    pub struct Input {
        pub races: Vec<Race>,
        pub actual_race: Race
    }

    #[derive(Debug)]
    pub struct Race {
        pub time: u64,
        pub record_distance: u64
    }


    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(
            line("Time:" " "+ repeat_sep(u32, " "+))
            line("Distance:" " "+ repeat_sep(u32, " "+))
        );

        let raw_data = read_to_string(filename)?;
        let parsed = parser.parse(&raw_data).context("parse error")?;

        let (_, times) = parsed.0;
        let (_, distance) = parsed.1;
        assert_eq!(times.len(), distance.len());

        let races: Vec<Race> = times.into_iter().zip(distance.into_iter())
            .map(|(time, record_distance)| Race {
                time: time as u64,
                record_distance: record_distance as u64
            })
            .collect();

        let mut actual_time = String::new();
        let mut actual_distance = String::new();
        races.iter().for_each(|r| {
            actual_time.push_str(r.time.to_string().as_str());
            actual_distance.push_str(r.record_distance.to_string().as_str());
        });

        Ok(Input {
            races: races,
            actual_race: Race {
                time: actual_time.parse().context("actual_time not a number")?,
                record_distance: actual_distance.parse().context("actual_distance not a number")?
            }
        })
    }
}

/// Your toy boat has a starting speed of zero millimeters per millisecond. For each whole
/// millisecond you spend at the beginning of the race holding down the button, the boat's speed
/// increases by one millimeter per millisecond.
fn distance_traveled(x: &u64, race_time: &u64) -> u64 {
    let travel_time = race_time - x;
    let start_speed = x;
    start_speed * travel_time // x * (race_time - x)
}

fn win_possible_interval(min_distance: &u64, race_time: &u64) -> RangeInclusive<u64> {
    // f(x) = x * (T-x)         where x = hold time and T is total race time
    //  we are only interested in solutions where we would win, so:
    // f(x) = (x * (T-x)) - D   where D is the record to beat
    //  solving for 0 gives us two solutions (https://www.wolframalpha.com/input?i=0+%3D+%28x+*+%28T-x%29%29+-+r)
    // x = 1/2 (T - sqrt(T^2 - 4 D))
    // x = 1/2 (sqrt(T^2 - 4 D) + T)

    let sqrt = |n: u64| (n as f64).sqrt();

    let inner = sqrt(race_time.pow(2) - 4 * min_distance);
    let race_time = *race_time as f64;

    let result_1 = 0.5 * (race_time - inner);
    let result_2 = 0.5 * (inner + race_time);

    (result_1.ceil().round() as u64)..=(result_2.floor() as u64)
}

fn winning_wait_times(race: &Race) -> u32 {

    (0..race.time)
        .map(|hold_time| distance_traveled(&hold_time, &race.time))
        .filter(|distance| distance > &race.record_distance)
        .count() as u32
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;

    let total = input.races.iter()
        .map(winning_wait_times)
        .fold(1_u32, |total, winning_waits| total * winning_waits);
    Ok(total)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    Ok(winning_wait_times(&input.actual_race))
}

fn main() -> Result<()> {
    simple_log::quick!("info");
    info!("Result part 1: {}", solve_part_1("src/day_06/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_06/input.txt")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{distance_traveled, solve_part_1, solve_part_2, win_possible_interval};

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_06/test_input.txt").unwrap();
        assert_eq!(result, 288);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_06/test_input.txt").unwrap();
        assert_eq!(result, 71503);
    }

    #[test]
    fn test_distance_function() {
        assert_eq!(distance_traveled(&0, &7), 0);
        assert_eq!(distance_traveled(&1, &7), 6);
        assert_eq!(distance_traveled(&2, &7), 10);
        assert_eq!(distance_traveled(&3, &7), 12);
        assert_eq!(distance_traveled(&4, &7), 12);
        assert_eq!(distance_traveled(&5, &7), 10);
        assert_eq!(distance_traveled(&6, &7), 6);
        assert_eq!(distance_traveled(&7, &7), 0);
    }

    #[test]
    fn test_win_possible_interval() {
        assert_eq!(win_possible_interval(&9, &7), 2..=5);
        assert_eq!(win_possible_interval(&40, &15), 4..=11);
        assert_eq!(win_possible_interval(&200, &30), 10..=20);
    }
}