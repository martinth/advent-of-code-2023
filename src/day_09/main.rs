use std::collections::VecDeque;
use anyhow::{anyhow, Context, Result};
use itertools::rev;

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

//
// pub struct DeltaIter<I> {
//     inner: I
// }
//
// impl<I> Iterator for DeltaIter<I> where I: Iterator<Item=u32> {
//     type Item = I::Item;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let last_0 = self.inner.next().expect("one more element");
//         let last_1 = self.inner.next().expect("one more element");
//
//         println!(" yield {} - {} -> {}", last_0, last_1, last_0 - last_1);
//
//         Some(last_0 - last_1)
//     }
// }

fn predict_next(dataset: Vec<i32>) -> Result<i32> {
    let last = dataset.last().expect("last element").clone();
    let next = dataset.windows(2)
        .map(|window| {
            //println!("win {:?} -> {}", window, window[1] - window[0]);
            window[1] - window[0]
        })
        .collect::<Vec<i32>>();


    //println!("{:?}, {}", next, last);

    let mut rev_iter = next.iter().rev();
    let last_next_0 = rev_iter.next().expect("last elem");

    if next.iter().all(|v| v == &0) {
        Ok(last + last_next_0)
    } else {
        let child = predict_next(next.clone()).unwrap();
        println!("{:?}...{}", next, child);

        Ok(last + child)
    }
}


// too high: 2175229208
// 2175229206
fn solve_part_1(filename: &str) -> Result<i128> {
    let input = parse::parse_input(filename)?;

    let mut total = 0_i128;
    for dataset in input.datasets {
        let next = predict_next(dataset.clone()).context("predict works")?;
        println!("{:?}", dataset);
        println!("---");
        total += next as i128;
    }

    Ok(total)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    println!("{:?}", input);

    todo!()
}

fn main() -> Result<()> {
    simple_log::quick!("info");
    let r1 = solve_part_1("src/day_09/input.txt")?;
    assert_eq!(2175229206, r1);
    info!("Result part 1: {}", r1);
    //info!("Result part 2: {}", solve_part_2("src/day_09/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{predict_next, solve_part_1, solve_part_2};

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_09/test_input.txt").unwrap();
        assert_eq!(result, 114);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_09/test_input.txt").unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_predict_next() {
        // assert_eq!(Some(8), predict_next(&vec![5, 6, 7]));
        // assert_eq!(Some(10), predict_next(&vec![4, 6, 8]));
        // assert_eq!(Some(40), predict_next(&vec![10, 20, 30]));

        assert_eq!(18, predict_next(vec![0, 3, 6, 9, 12, 15]).unwrap());
        println!("---");
        assert_eq!(28, predict_next(vec![1, 3, 6, 10, 15, 21]).unwrap());
        println!("---");
        assert_eq!(68, predict_next(vec![10, 13, 16, 21, 30, 45]).unwrap());
    }
}