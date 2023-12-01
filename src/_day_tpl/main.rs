use advent_of_code_2023::common::read_valid_lines;
use anyhow::{Result, Context};


fn solve(filename: &str) -> Result<u32> {
    let lines = read_valid_lines(filename);

    todo!()
}

fn main() {
    let result = solve("src/day_xx/input.txt");

    println!("Result: {}", result.expect("result should exist"));
}


#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn solve_test_input() {
        let result = solve("src/day_xx/test_input.txt").unwrap();
        assert_eq!(result, 42);
    }
}