extern crate core;

use std::collections::HashMap;
use std::fs::{read_to_string};
use anyhow::{Result, Context};
use aoc_parse::{parser, prelude::*};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug)]
enum Number {
    Emtpy,
    PartNumber(String)
}

#[derive(Debug)]
enum Part {
    Emtpy,
    Gear,
    OtherPart,
}

#[derive(Debug)]
struct Plan {
    numbers: Vec<Vec<Number>>,
    parts: Vec<Vec<Part>>,
}

impl Number {

    /// Produce a vector of all adjacent, surrounding points (so 1 point bounding box)
    /// relative to the location given. For PartNumber the location expected to be the
    /// top left coordinate.
    /// Function makes sure to not produce negative locations but does not know about
    /// max global locations.
    fn adjacent(self: &Self, base: Point) -> Box<dyn Iterator<Item=Point> + '_> {
        let safe_base = |n: usize | if n == 0 {
            0
        } else {
            n - 1
        };

        let xs = safe_base(base.x)..(base.x + self.len() + 1);
        let ys = safe_base(base.y)..(base.y + 2);

        let all_points = ys
            .flat_map(move |y| {
                xs.clone().map(move |x| Point {x, y})
            })
            // filter out points that match the text
            .filter(move |p| if p.y != base.y {
                true
            } else {
                p.x < base.x || p.x >= (base.x + self.len())
            });

        Box::new(all_points)
    }

    fn len(self: &Self) -> usize {
        match self {
            Number::Emtpy => 1,
            Number::PartNumber(s) => s.len()
        }
    }
}

fn parse_input(filename: &str) -> Result<Plan> {
    let part_parser = parser!({
        "." => Part::Emtpy,
        "*" => Part::Gear,
        digit => Part::Emtpy,
        any_char => Part::OtherPart,
    });
    let number_parser = parser!({
        s:string(digit+) => Number::PartNumber(s),
        any_char => Number::Emtpy
    });

    let raw_data = read_to_string(filename)?;
    let parts = parser!(lines(part_parser+))
        .parse(&raw_data)
        .context("error parsing parts")?;
    let numbers = parser!(lines(number_parser+))
        .parse(&raw_data)
        .context("error parsing numbers")?;

    Ok(Plan {parts, numbers})
}

fn solve(filename: &str) -> Result<(u32, u32)> {
    let plan = parse_input(filename)?;

    // we assume a rectangular plan
    let max_x = plan.parts[0].len() - 1;
    let max_y = plan.parts.len() - 1;

    let mut total_of_part_numbers = 0;
    let mut gears: HashMap<Point, Vec<u32>> = HashMap::new();

    for (y, row) in plan.numbers.iter().enumerate() {
        let mut x = 0;
        for number in row {
            if let Number::PartNumber(s) = number {
                let part_number: u32 = s.parse()?;
                let adjacent_points = number.adjacent(Point {x, y});
                let valid_adjacent = adjacent_points
                    .filter(|p| p.x <= max_x && p.y <= max_y);

                for p in valid_adjacent {
                    match plan.parts[p.y][p.x] {
                        Part::Gear => {
                            total_of_part_numbers += part_number;

                            gears.entry(p.clone())
                                .and_modify(|v| v.push(part_number))
                                .or_insert(vec![part_number]);
                        },
                        Part::OtherPart => {
                            total_of_part_numbers += part_number;
                        }
                        _ => {} // do nothing
                    }
                }

            }
            x += number.len()
        }
    }

    let gear_rations = gears.values()
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts[0] * parts[1])
        .sum();

    Ok((total_of_part_numbers, gear_rations))
}


fn main() -> Result<()> {
    let (r1, r2) = solve("src/day_03/input.txt")?;
    println!("Result part 1: {}", r1);
    println!("Result part 2: {}", r2);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{Number, Point, solve};

    #[test]
    fn solve_test_input_1() {
        let result = solve("src/day_03/test_input.txt").unwrap();
        assert_eq!(result, (4361, 467835));
    }

    #[test]
    fn test_adjacent_generator() {

        assert_eq!(
            Number::PartNumber("1".to_string()).adjacent(Point {x: 3, y: 3 }).len(),
            8
        );
        assert_eq!(
            Number::PartNumber("123".to_string()).adjacent(Point {x: 3, y: 3 }).len(),
            12
        );
        assert_eq!(
            Number::PartNumber("1".to_string()).adjacent(Point {x: 0, y: 0 }).len(),
            3
        );
        assert_eq!(
            Number::PartNumber("467".to_string()).adjacent(Point {x: 0, y: 0 }).len(),
            5
        );

        assert_eq!(
            Number::Emtpy.adjacent(Point {x: 3, y: 3 }).len(),
            8
        );
        assert_eq!(
            Number::Emtpy.adjacent(Point {x: 0, y: 0 }).len(),
            3
        );
    }
}