use std::fs::{read_to_string};
use anyhow::{Result, Context};
use aoc_parse::{parser, prelude::*};


#[derive(Debug)]
enum PlanItem {
    Emtpy,
    EnginePart,
    PartNumber(String)
}

fn parse_input(filename: &str) -> Result<Vec<Vec<PlanItem>>> {
    let item_parser = parser!({
        "." => PlanItem::Emtpy,
        "*" => PlanItem::EnginePart,
        "#" => PlanItem::EnginePart,
        "+" => PlanItem::EnginePart,
        "$" => PlanItem::EnginePart,
        s:string(digit+) => PlanItem::PartNumber(s),

    });
    let parser = parser!(lines(item_parser+));

    let raw_data = read_to_string(filename)?;
    parser.parse(&raw_data).context("parse error")
}


fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse_input(filename)?;
    println!("{:?}", input);

    todo!()
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse_input(filename)?;
    println!("{:?}", input);

    todo!()
}

fn main() -> Result<()> {
    println!("Result part 1: {}", solve_part_1("src/day_03/input.txt")?);
    println!("Result part 2: {}", solve_part_2("src/day_03/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_03/test_input.txt").unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_03/test_input.txt").unwrap();
        assert_eq!(result, 42);
    }
}