use anyhow::{Result};
use crate::parse::Input;

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
        Joker,
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

    #[derive(Debug, Eq, PartialEq, Ordinalize, PartialOrd, Ord, Clone)]
    pub enum HandType {
        HighCard,
        OnePair,
        TwoPairs,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(Debug)]
    pub struct Hand {
        pub cards: [Card; 5],
        pub hand_type: HandType,
        pub type_with_joker: Option<HandType>,
        pub bid: u32
    }

    pub fn type_with_joker(hand_type: &HandType, cards: &Vec<Card>) -> HandType {
        let joker_count: u8 = cards.iter()
            .filter_map(|c| match c {
                Card::Joker => Some(1),
                _ => None
            })
            .sum();

        match hand_type {
            HandType::FourOfAKind  => match joker_count {
                4 => HandType::FiveOfAKind, // must be 4 jokers, so other card makes it 5
                1 => HandType::FiveOfAKind,
                0 => hand_type.clone(),
                _ => panic!("unexpected joker count {} in {:?}", joker_count, cards)
            }
            HandType::ThreeOfAKind => {
                match joker_count {
                    // the three cards must be jokers already, the other two cards must be different
                    // otherwise we had FullHouse
                    3 => HandType::FourOfAKind,
                    2 => HandType::FiveOfAKind,
                    1 => HandType::FourOfAKind,
                    0 => hand_type.clone(),
                    _ => panic!("unexpected joker count {} in {:?}", joker_count, cards)
                }
            }
            HandType::TwoPairs => match joker_count {
                2 => HandType::FourOfAKind,
                1 => HandType::FullHouse,
                0 => hand_type.clone(),
                _ => panic!("unexpected joker count {} in {:?}", joker_count, cards)
            },
            HandType::OnePair => match joker_count {
                3 => HandType::FiveOfAKind,
                2 => HandType::ThreeOfAKind, // the two cards must be the joker, otherwise we had TwoPair
                1 => HandType::ThreeOfAKind,
                0 => hand_type.clone(),
                _ => panic!("unexpected joker count {} in {:?}", joker_count, cards)
            },
            HandType::HighCard => match joker_count {
                4 => HandType::FiveOfAKind,
                3 => HandType::FourOfAKind,
                2 => HandType::ThreeOfAKind,
                1 => HandType::OnePair,
                0 => hand_type.clone(),
                _ => panic!("unexpected joker count {} in {:?}", joker_count, cards)
            }
            HandType::FullHouse  => match joker_count {
                3 => HandType::FiveOfAKind,
                2 => HandType::FiveOfAKind,
                0 => hand_type.clone(),
                _ => panic!("unexpected joker count {} in {:?}", joker_count, cards)
            },
            HandType::FiveOfAKind => hand_type.clone(), // hand is full, no way to improve it
        }

    }

    impl Hand {
        pub fn new(cards: Vec<Card>, bid: u32, with_joker: bool) -> Self {
            // get a copy of the cards as array for passing on
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
                (1, _) => HandType::HighCard,
                (_ ,_) => panic!("Unexpected pair counts: {}/{}", longest, second_longest)
            };
            let type_with_joker = if with_joker {
                Some(type_with_joker(&hand_type, &cards))
            } else {
                None
            };

            Hand {
                cards: cards_org_order,
                hand_type,
                type_with_joker,
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
            let by_type = if let Some(with_joker) = &self.type_with_joker {
                let other_with_joker = other.type_with_joker.as_ref()
                    .expect("other to have joker type");
                with_joker.cmp(other_with_joker)
            } else {
                self.hand_type.cmp(&other.hand_type)
            };


            match by_type {
                Ordering::Less => Some(by_type),
                Ordering::Greater => Some(by_type),
                Ordering::Equal => {
                    for (c_self, c_other) in self.cards.iter().zip(other.cards.iter()) {
                        if c_self != c_other {
                            return c_self.partial_cmp(c_other);
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
                    Card::Joker => write!(f, "*"),
                }?;
            }
            Ok(())
        }
    }


    #[derive(Debug)]
    pub struct Input {
        pub hands: Vec<Hand>
    }

    pub(crate) fn parse_hand(hand: &str, with_joker: bool) -> Hand {
        let parser = parser!(cards:{
            "A" => Card::Ass,
            "K" => Card::King,
            "Q" => Card::Queen,
            "J" => if with_joker {
                Card::Joker
            } else {
                Card::Jack
            },
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

        Hand::new(parser.parse(hand).expect("valid hand"), 0, with_joker)
    }


    pub fn parse_input(filename: &str, with_joker: bool) -> Result<Input> {
        let parser = parser!(lines(cards:{
            "A" => Card::Ass,
            "K" => Card::King,
            "Q" => Card::Queen,
            "J" => if with_joker {
                Card::Joker
            } else {
                Card::Jack
            },
            "T" => Card::Ten,
            "9" => Card::Nine,
            "8" => Card::Eight,
            "7" => Card::Seven,
            "6" => Card::Six,
            "5" => Card::Five,
            "4" => Card::Four,
            "3" => Card::Three,
            "2" => Card::Two
        }+ " " bid:u32 => Hand::new(cards, bid, with_joker)));

        let raw_data = read_to_string(filename)?;
        let hands = parser.parse(&raw_data).context("parse error")?;

        Ok(Input { hands })
    }
}

fn solve_common(mut input: Input) -> Result<u32> {
    input.hands.sort();

    let mut total_winnings = 0_u32;
    for (idx, hand) in input.hands.iter().enumerate() {
        if let Some(with_joker_type) = &hand.type_with_joker {
            println!("{:?} {} is rank {} ", with_joker_type, hand, &idx + 1);
        } else {
            println!("{} is rank {} ", hand, &idx + 1);
        }
        total_winnings += &hand.bid * (idx as u32 + 1);
    }

    Ok(total_winnings)
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename, false)?;
    solve_common(input)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename, true)?;

    solve_common(input)
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_07/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_07/input.txt")?);
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

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_07/test_input.txt").unwrap();
        assert_eq!(result, 5905);
    }


    #[test]
    fn test_hand_type_part_1() {
        let input = parse_input("src/day_07/test_input.txt", false).unwrap();

        assert_eq!(input.hands[0].hand_type, HandType::OnePair);
        assert_eq!(input.hands[1].hand_type, HandType::ThreeOfAKind);
        assert_eq!(input.hands[2].hand_type, HandType::TwoPairs);
        assert_eq!(input.hands[3].hand_type, HandType::TwoPairs);
        assert_eq!(input.hands[4].hand_type, HandType::ThreeOfAKind);
    }

    #[test]
    fn test_tie_breaking_part_1() {
        assert!(parse_hand("AAAAQ", false) > parse_hand("AAAAJ", false));
        assert!(parse_hand("AAAQQ", false) > parse_hand("AAAJJ", false));
        assert!(parse_hand("264AJ", false) < parse_hand("269J8", false))
    }

    #[test]
    fn hand_ordering_part_1() {
        assert!(parse_hand("T55J5", false) > parse_hand("KTJJT", false));
        assert!(parse_hand("KK677", false) > parse_hand("KTJJT", false));
    }

    #[test]
    fn test_type_with_joker() {
        let test_cases = vec![
            ("T55J5", HandType::FourOfAKind),
            ("T55J5", HandType::FourOfAKind),
            ("KTJJT", HandType::FourOfAKind),
            ("QQQJA", HandType::FourOfAKind),
            ("KK677", HandType::TwoPairs),
        ];

        for (cards, expected_type) in test_cases {
            let hand = parse_hand(cards, true);
            assert_eq!(hand.type_with_joker.clone().unwrap(), expected_type,
                       "{} has type {:?}, expected {:?}", hand, hand.type_with_joker, expected_type);
        }
    }
}