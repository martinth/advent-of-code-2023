use std::cmp::{max};
use advent_of_code_2023::common::read_valid_lines;
use anyhow::{Result, Context};
use aoc_parse::{parser, prelude::*};

#[derive(Debug)]
enum Color {
    Blue,
    Red,
    Green
}

#[derive(Debug)]
struct DrawItem {
    color: Color,
    amount: u32
}

#[derive(Debug)]
struct Draw {
    items: Vec<DrawItem>
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>
}

fn parse_line(line: String) -> Result<Game> {
    let game_p = parser!("Game " u32 ": ");
    let color_p = parser!({
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue
    });
    let draw_item_p = parser!(amount: u32 " " color:color_p => DrawItem { color, amount });
    let draw_p = parser!(items:repeat_sep(draw_item_p, ", ") => Draw { items });
    let line_p = parser!(id:game_p draws:repeat_sep(draw_p, "; ") => Game { id, draws });
    line_p.parse(&line).context("parsing failed")
}

fn is_possible_item(draw_item: &DrawItem) -> bool {
    match draw_item {
        DrawItem{ color: Color::Red, amount} => amount <= &12,
        DrawItem{ color: Color::Green, amount} => amount <= &13,
        DrawItem{ color: Color::Blue, amount} => amount <= &14
    }
}

fn is_possible(game: &Game) -> bool {
    for draw in &game.draws {
        for draw_item in &draw.items {
            if !is_possible_item(draw_item) {
                return false
            }
        }
    }
    true
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let lines = read_valid_lines(filename);
    let mut id_sum = 0;
    for line in lines {
        let game = parse_line(line)?;
        if is_possible(&game) {
            id_sum += game.id
        }
    }

    Ok(id_sum)
}

fn power_of_draws(draws: Vec<Draw>) -> u32 {
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;

    for draw in draws {
        for item in draw.items {
            match item.color {
                Color::Red => max_red = max(max_red, item.amount),
                Color::Blue => max_blue = max(max_blue, item.amount),
                Color::Green => max_green = max(max_green, item.amount)
            }
        }
    }

    max_red * max_green * max_blue
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let lines = read_valid_lines(filename);
    let mut sum_of_powers = 0;
    for line in lines {
        let game = parse_line(line)?;
        let power = power_of_draws(game.draws);
        sum_of_powers += power
    }
    Ok(sum_of_powers)
}

fn main() -> Result<()> {
    println!("Result part 1: {}", solve_part_1("src/day_02/input.txt")?);
    println!("Result part 2: {}", solve_part_2("src/day_02/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_02/test_input.txt").unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_02/test_input.txt").unwrap();
        assert_eq!(result, 2286);
    }
}