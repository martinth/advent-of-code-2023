use anyhow::{Result};

#[macro_use]
extern crate simple_log;
extern crate core;

mod parse {
    use std::cmp::Ordering;
    use std::fmt;
    use std::fmt::Formatter;
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use itertools::Itertools;
    use enum_ordinalize::Ordinalize;

    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Ordinalize, Copy, Clone)]
    pub enum Card {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ass,
    }


    #[derive(Debug, Eq, PartialEq)]
    pub enum HandType {
        HighCard(Card),
        OnePair,
        TwoPairs,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    impl HandType {
        fn ordinal(self: &Self) -> i8 {
            match self {
                HandType::HighCard(card) => card.ordinal(),
                HandType::OnePair => 20,
                HandType::TwoPairs => 21,
                HandType::ThreeOfAKind => 22,
                HandType::FullHouse => 23,
                HandType::FourOfAKind => 24,
                HandType::FiveOfAKind => 35,
            }
        }
    }

    impl PartialOrd for HandType {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let result = self.ordinal().partial_cmp(&other.ordinal());
            println!("HandType.partial_cmp: {:?} {:?} {:?}", self.ordinal(), result, other.ordinal());
            result
        }
    }
    impl Ord for HandType {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    #[derive(Debug)]
    pub struct Hand {
        pub cards: [Card; 5],
        pub hand_type: HandType,
        pub bid: u32
    }

    impl Hand {
        pub fn new(cards: Vec<Card>, bid: u32) -> Self {
            // get a copy of the cards since we need to store them in original order
            let cards_org_order: [Card; 5] = cards.clone().try_into()
                .expect("infallible");

            let counted = cards.iter().counts_by(|c| c.ordinal());
            let mut groups = counted.values().sorted().rev();
            let longest = groups.next().expect("no longest");
            let second_longest = groups.next().unwrap_or(&0);

            let hand_type = match (longest, second_longest) {
                (5, 0) => HandType::FiveOfAKind,
                (4, 1) => HandType::FourOfAKind,
                (3, 2) => HandType::FullHouse,
                (3, 1) => HandType::ThreeOfAKind,
                (2, 2) => HandType::TwoPairs,
                (2, 1) => HandType::OnePair,
                (1, _) => HandType::HighCard(*cards.iter().max().expect("largest card")),
                (_ ,_) => panic!("Unexpected pair counts: {}/{}", longest, second_longest)
            };

            Hand {
                cards: cards_org_order,
                hand_type,
                bid
            }
        }
    }

    impl PartialEq<Self> for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.cmp(other) == Ordering::Equal
        }
    }
    impl Eq for Hand {}

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let by_type = self.hand_type.cmp(&other.hand_type);
            println!("Hand.partial_cmp - {:?} {:?} {:?}",  self.hand_type, by_type, other.hand_type);

            match by_type {
                Ordering::Less => Some(by_type),
                Ordering::Greater => Some(by_type),
                Ordering::Equal => {
                    for (c_self, c_other) in self.cards.iter().zip(other.cards.iter()) {
                        if c_self != c_other {
                            let result = c_self.partial_cmp(c_other);
                            println!("  cards {}: {:?} {:?} {:?} ", self, c_self, result, c_other);
                            return result
                        }
                    }
                    Some(Ordering::Equal)
                }
            }
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
           self.partial_cmp(other).unwrap()
        }
    }

    impl fmt::Display for Hand {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for card in self.cards {
                match card {
                    Card::Ass => write!(f, "A"),
                    Card::King => write!(f, "K"),
                    Card::Queen => write!(f, "Q"),
                    Card::Jack => write!(f, "J"),
                    Card::Ten => write!(f, "T"),
                    Card::Nine => write!(f, "9"),
                    Card::Eight => write!(f, "8"),
                    Card::Seven => write!(f, "7"),
                    Card::Six => write!(f, "6"),
                    Card::Five => write!(f, "5"),
                    Card::Four => write!(f, "4"),
                    Card::Three => write!(f, "3"),
                    Card::Two => write!(f, "2"),
                }?;
            }
            Ok(())
        }
    }


    #[derive(Debug)]
    pub struct Input {
        pub hands: Vec<Hand>
    }

    pub fn parse_hand(hand: &str) -> Hand {
        let parser = parser!(cards:{
            "A" => Card::Ass,
            "K" => Card::King,
            "Q" => Card::Queen,
            "J" => Card::Jack,
            "T" => Card::Ten,
            "9" => Card::Nine,
            "8" => Card::Eight,
            "7" => Card::Seven,
            "6" => Card::Six,
            "5" => Card::Five,
            "4" => Card::Four,
            "3" => Card::Three,
            "2" => Card::Two
        }+);

        Hand::new(parser.parse(hand).expect("valid hand"), 0)
    }


    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(lines(cards:{
            "A" => Card::Ass,
            "K" => Card::King,
            "Q" => Card::Queen,
            "J" => Card::Jack,
            "T" => Card::Ten,
            "9" => Card::Nine,
            "8" => Card::Eight,
            "7" => Card::Seven,
            "6" => Card::Six,
            "5" => Card::Five,
            "4" => Card::Four,
            "3" => Card::Three,
            "2" => Card::Two
        }+ " " bid:u32 => Hand::new(cards, bid)));

        let raw_data = read_to_string(filename)?;
        let hands = parser.parse(&raw_data).context("parse error")?;

        Ok(Input { hands })
    }
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let mut input = parse::parse_input(filename)?;

    input.hands.sort();

    let mut total_winnings = 0_u32;
    for (idx, hand) in input.hands.iter().enumerate() {
        println!("{} is rank {}", hand, &idx + 1);
        total_winnings += &hand.bid * (idx as u32 + 1);
    }

    Ok(total_winnings)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    println!("{:?}", input);

    todo!()
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    // wrong:
    // 251544771
    // 249643397
    // 249455729
    info!("Result part 1: {}", solve_part_1("src/day_07/input.txt")?);
    //info!("Result part 2: {}", solve_part_2("src/day_07/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};
    use crate::parse::{HandType, parse_hand, parse_input};

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_07/test_input.txt").unwrap();
        assert_eq!(result, 6440);
    }

    // #[test]
    // fn solve_test_input_2() {
    //     let result = solve_part_2("src/day_07/test_input.txt").unwrap();
    //     assert_eq!(result, 42);
    // }
    #[test]
    fn fail() {
        parse_hand("56566");
        assert!(false)
    }

    #[test]
    fn test_hand_type() {
        let input = parse_input("src/day_07/test_input.txt").unwrap();

        assert_eq!(input.hands[0].hand_type, HandType::OnePair);
        assert_eq!(input.hands[1].hand_type, HandType::ThreeOfAKind);
        assert_eq!(input.hands[2].hand_type, HandType::TwoPairs);
        assert_eq!(input.hands[3].hand_type, HandType::TwoPairs);
        assert_eq!(input.hands[4].hand_type, HandType::ThreeOfAKind);
    }


    #[test]
    fn test_tie_breaking() {
        assert!(parse_hand("AAAAQ") > parse_hand("AAAAJ"));
        assert!(parse_hand("AAAQQ") > parse_hand("AAAJJ"));
        assert!(parse_hand("264AJ") > parse_hand("269J8"))
    }

    #[test]
    fn hand_ordering() {
        assert!(parse_hand("T55J5") > parse_hand("KTJJT"));
        assert!(parse_hand("KK677") > parse_hand("KTJJT"));
    }
}