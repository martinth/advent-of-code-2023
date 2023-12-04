use advent_of_code_2023::common::read_valid_lines;
use anyhow::{Result, Context};
use regex::{Regex};

#[macro_use]
extern crate simple_log;

fn calibration_value_words(line: String) -> Result<u32> {
    let translator = |num_str: &str| match num_str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        other => panic!("Unexpected input: {}", other)
    };

    // replace first number word
    let base_regex = r"(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)";
    let first = Regex::new(base_regex)
        .expect("valid regex")
        .captures(&line)
        .and_then(|caps| caps.get(1))
        .map(|mat| translator(&line[mat.range()]))
        .context("not even one digit found")?;

    let last = Regex::new(format!(".*{}", base_regex).as_str())
        .expect("valid regex")
        .captures(&line)
        .and_then(|caps| caps.get(1))
        .map(|met| translator(&line[met.range()]))
        .unwrap_or(first);

    Ok((first * 10) + last)
}

fn calibration_value(line: String) -> Result<u32> {

    let mut digits = line.chars().filter(|c| c.is_digit(10));
    let first = digits.next()
        .context("no first digit")?
        .to_digit(10).unwrap();
    let last = digits.last()
        .map(|c| c.to_digit(10).unwrap())
        .unwrap_or(first);

    Ok((first * 10) + last)
}

/// The newly-improved calibration document consists of lines of text; each line originally
/// contained a specific calibration value that the Elves now need to recover. On each line, the
/// calibration value can be found by combining the first digit and the last digit (in that order)
/// to form a single two-digit number.
fn solve_01(filename: &str) -> Result<u32> {
    let lines = read_valid_lines(filename);
    let mut sum: u32 = 0;
    for line in lines {
        sum += calibration_value(line)?
    }
    Ok(sum)
}

/// Your calculation isn't quite right. It looks like some of the digits are actually spelled out
/// with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid
/// "digits".
fn solve_02(filename: &str) -> Result<u32> {
    let lines = read_valid_lines(filename);
    let mut sum: u32 = 0;
    for line in lines {
        sum += calibration_value_words(line)?
    }
    Ok(sum)
}

fn main() {
    simple_log::quick!("info");

    let result_1 = solve_01("src/day_01/input.txt");
    let result_2 = solve_02("src/day_01/input.txt");

    info!("Result part 1: {}", result_1.expect("result 1"));
    info!("Result part 2: {}", result_2.expect("result 2"));
}


#[cfg(test)]
mod tests {
    use crate::solve_01;
    use crate::solve_02;
    use crate::calibration_value_words;

    #[test]
    fn solve_test_input_part_01() {
        let result = solve_01("src/day_01/test_input_part_01.txt").unwrap();
        assert_eq!(result, 142);
    }

    #[test]
    fn solve_test_input_part_02() {
        let result = solve_02("src/day_01/test_input_part_02.txt").unwrap();
        assert_eq!(result, 281);
    }

    #[test]
    fn test_calibration_value_words() {
        assert_eq!(calibration_value_words("two".to_string()).unwrap(), 22);
        assert_eq!(calibration_value_words("2".to_string()).unwrap(), 22);
        assert_eq!(calibration_value_words("21".to_string()).unwrap(), 21);
        assert_eq!(calibration_value_words("twone45sevenine".to_string()).unwrap(), 29);
        assert_eq!(calibration_value_words("one23".to_string()).unwrap(), 13);
        assert_eq!(calibration_value_words("onetwothree".to_string()).unwrap(), 13);
        assert_eq!(calibration_value_words("eighthree".to_string()).unwrap(), 83);
        assert_eq!(calibration_value_words("sevenine".to_string()).unwrap(), 79);
    }
}