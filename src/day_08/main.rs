use std::iter::repeat;
use anyhow::{Result};
use crate::parse::Move;

#[macro_use]
extern crate simple_log;

mod parse {
    use std::collections::HashMap;
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;

    #[derive(Debug)]
    pub enum Move {
        Left,
        Right
    }

    #[derive(Debug)]
    pub struct Input {
        pub moves: Vec<Move>,
        pub nodes: HashMap<String, Node>
    }

    #[derive(Debug)]
    pub struct Node {
        pub left: String,
        pub right: String,
    }

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(
            line({
                "L" => Move::Left,
                "R" => Move::Right
            }+)
            line("")
            lines(name:string(upper+) " = (" left:string(upper+) ", " right:string(upper+) ")")
        );

        let raw_data = read_to_string(filename)?;
        let (moves, _, nodes) = parser.parse(&raw_data).context("parse error")?;


        let nodes = HashMap::from_iter(nodes.into_iter().map(|(name, left, right)| {
            let node = Node {
                left,
                right
            };
            (name.clone(), node)
        }));


        Ok(Input {
            moves,
            nodes
        })
    }
}

#[derive(Debug)]
pub struct Shortcut {
    left: Option<String>,
    right: Option<String>,
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;


    let mut steps = 0_u32;
    let mut moves = repeat(&input.moves)
        .flat_map(|moves| moves.iter());

    let mut current = &"AAA".to_string();
    while current != &"ZZZ".to_string() {
        let node = input.nodes.get(current)
            .expect("node to exist");
        let action = moves.next().expect("another move");
        let next = match action  {
            Move::Left => &node.left,
            Move::Right => &node.right,
        };

        debug!("At {} going {:?} to {}", current, action, next);

        current = next;
        steps += 1;
    }

    Ok(steps)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    println!("{:?}", input);

    todo!()
}

fn main() -> Result<()> {
    simple_log::quick!("debug");
    info!("Result part 1: {}", solve_part_1("src/day_08/input.txt")?);
    //info!("Result part 2: {}", solve_part_2("src/day_xx/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};

    #[test]
    fn solve_test_input_1_0() {
        let result = solve_part_1("src/day_08/test_input_0.txt").unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn solve_test_input_1_1() {
        let result = solve_part_1("src/day_08/test_input_1.txt").unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn solve_test_input_() {
        let result = solve_part_2("src/day_08/test_input.txt").unwrap();
        assert_eq!(result, 42);
    }
}