use advent_of_code_2023::common::read_valid_lines;
use anyhow::{Result, Context};
use regex::{Regex};


fn word_replacement(line: String) -> String {
    let replacer = |num_str: &str| match num_str {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        other => panic!("Unexpected word: {}", other)
    };

    // replace first number word
    let digit_word_re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)")
        .expect("valid regex");

    let mut copy = line.clone();
    let mut matches = digit_word_re.find_iter(&line);
    let first_match = matches.next();
    let last_match = matches.last();

    let mut line2 = if let Some(last_match) = last_match {
        copy.replace_range(last_match.range(), replacer(last_match.as_str()));
        copy
    } else {
        copy
    };

    let mut line2 = if let Some(first_match) = first_match {
        line2.replace_range(first_match.range(), replacer(first_match.as_str()));
        line2
    } else {
        line2
    };



    line2
}

fn calibration_value(line: String) -> Result<u32> {

    let mut digits = line.chars().filter(|c| c.is_digit(10));
    let first = digits.next()
        .context("no first digit")?
        .to_digit(10).unwrap();
    let last = digits.last()
        .map(|c| c.to_digit(10).unwrap())
        .unwrap_or(first);

    // debugging
    println!("{} -> {}{}", line, first, last);

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
        print!("{} -> ", line);
        sum += calibration_value(word_replacement(line))?
    }
    Ok(sum)
}

fn main() {
    let result_1 = solve_01("src/day_01/input.txt");
    let result_2 = solve_02("src/day_01/input.txt");

    println!("Result part 1: {}", result_1.expect("result 1"));

    // TODO: this is not giving the correct answer, according to the site but the test passes and it looks all correct
    println!("Result part 2: {}", result_2.expect("result 2"));
}


#[cfg(test)]
mod tests {
    use crate::solve_01;
    use crate::solve_02;
    use crate::word_replacement;

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
    fn test_word_replacement() {
        assert_eq!(word_replacement("twone45sevenine".to_string()), "2ne45seve9");
        assert_eq!(word_replacement("one23".to_string()), "123");
        assert_eq!(word_replacement("nothing".to_string()), "nothing");
        assert_eq!(word_replacement("onetwothree".to_string()), "1two3");
    }
}