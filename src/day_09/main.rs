use anyhow::{Context, Result};

#[macro_use]
extern crate simple_log;
extern crate core;

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;

    #[derive(Debug)]
    pub struct Input {
        pub(crate) datasets: Vec<Vec<i32>>
    }

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(lines(repeat_sep(i32, " ")));

        let raw_data = read_to_string(filename)?;
        let raw_parsed = parser.parse(&raw_data).context("parse error")?;

        Ok(Input {
            datasets: raw_parsed
        })
    }
}

fn predict(dataset: Vec<i32>) -> Result<(i32, i32)> {

    // derive the next row from the different between items
    let derived_row = dataset.windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i32>>();

    let first = dataset.get(0).context("last element")?;
    let last = dataset.last().context("last element")?;
    let last_derived = derived_row.last().context("no last elem")?;
    let first_derived = derived_row.first().context("no first elem")?;

    if derived_row.iter().all(|v| v == &0) {
        Ok((first - first_derived, last + last_derived))
    } else {
        let (predicted_prev, predicted_next) = predict(derived_row).unwrap();
        Ok((first - predicted_prev, last + predicted_next))
    }
}

fn solve_both_parts(filename: &str) -> Result<[i128; 2]> {
    let input = parse::parse_input(filename)?;

    let mut total_next = 0_i128;
    let mut total_prev = 0_i128;
    for dataset in input.datasets {
        let (prev, next) = predict(dataset.clone()).context("predict works")?;
        total_next += next as i128;
        total_prev += prev as i128;
    }

    Ok([total_prev, total_next])
}

fn main() -> Result<()> {
    simple_log::quick!("info");
    let [r2, r1] = solve_both_parts("src/day_09/input.txt")?;
    assert_eq!(2175229206, r1);
    info!("Result part 1: {}", r1);
    info!("Result part 2: {}", r2);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{solve_both_parts};

    #[test]
    fn solve_test_input() {
        let [result_2, result_1] = solve_both_parts("src/day_09/test_input.txt").unwrap();
        assert_eq!(result_1, 114);
        assert_eq!(result_2, 2);
    }
}