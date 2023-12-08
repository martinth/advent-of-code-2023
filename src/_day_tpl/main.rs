use anyhow::{Result, Context};
#[macro_use]
extern crate simple_log;

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;

    #[derive(Debug)]
    pub struct Input {

    }

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(lines(""));

        let raw_data = read_to_string(filename)?;
        let raw_parsed = parser.parse(&raw_data).context("parse error")?;

        Ok(Input {

        })
    }
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    println!("{:?}", input);

    todo!()
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    println!("{:?}", input);

    todo!()
}

fn main() -> Result<()> {
    simple_log::quick!("info");
    info!("Result part 1: {}", solve_part_1("src/day_xx/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_xx/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_xx/test_input.txt").unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_xx/test_input.txt").unwrap();
        assert_eq!(result, 42);
    }
}