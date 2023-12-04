use std::collections::HashMap;
use anyhow::Result;

#[macro_use]
extern crate simple_log;

mod parse {
    use std::collections::HashSet;
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;

    #[derive(Debug)]
    pub struct Card {
        pub id: u32,
        pub winning_numbers: HashSet<u32>,
        pub selected_numbers: HashSet<u32>
    }

    impl Card {
        pub fn num_matches(self: &Self) -> u32 {
            self.winning_numbers.intersection(&self.selected_numbers).count() as u32
        }
    }

    pub fn parse_input(filename: &str) -> Result<Vec<Card>> {

        let number_parser = parser!(nums:repeat_sep(u32, " "+) => nums);
        let line_parser = parser!("Card" " "+ id:u32 ":" " "+ winning_numbers:number_parser " |" " "+ drawn_numbers:number_parser => Card {
            id,
            winning_numbers: HashSet::from_iter(winning_numbers.into_iter()),
            selected_numbers: HashSet::from_iter(drawn_numbers.into_iter())
        });
        let parser = parser!(lines(line_parser));

        let raw_data = read_to_string(filename)?;
        parser.parse(&raw_data).context("parse error")
    }
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    let total: u32 = input.iter()
        .map(|card| card.num_matches())
        .filter(|matches| matches > &0)
        .map(|matches| 2_u32.pow(matches - 1))
        .sum();

    Ok(total)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    let mut total = input.len() as u32;
    let mut copies: HashMap<u32, u32> = HashMap::with_capacity(input.len() + 1);

    for card in input.into_iter() {
        let matches = card.num_matches();
        debug!("Original card {} has {} wins.", card.id, matches);

        for next_card_num in (card.id + 1)..(card.id + 1 + matches) {
            *copies.entry(next_card_num).or_insert(0) += 1;
            let copy_count = copies.get(&next_card_num).expect("card entry exists");
            debug!("  updated count for {} to {}", next_card_num, copy_count);
        }

        let copy_count = *copies.get(&card.id).unwrap_or(&0);
        debug!(" Card {} has {} copies.", card.id, copy_count);
        for _ in 0..copy_count {
            for next_card_num in (card.id + 1)..(card.id + 1 + matches) {
                *copies.entry(next_card_num).or_insert(0) += 1;
                let copy_count = copies.get(&next_card_num).expect("card entry exists");
                debug!("  updated count for {} to {}", next_card_num, copy_count);
            }
        }
    }

    total += copies.values().sum::<u32>();

    Ok(total)
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_04/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_04/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_04/test_input.txt").unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_04/test_input.txt").unwrap();
        assert_eq!(result, 30);
    }
}