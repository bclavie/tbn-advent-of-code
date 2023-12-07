use std::collections::HashMap;

fn main() {
    let inputs = include_str!("inputs/07");

    let mut hands: Vec<Hand> = inputs
        .split('\n')
        .map(|line| {
            let (hand_str, bid) = line.split_once(' ').unwrap();

            let mut cards = [Card::A; 5];
            for (i, c) in hand_str.chars().enumerate() {
                cards[i] = Card::from(c);
            }

            let mut card_counts = HashMap::<Card, u8>::new();
            for c in cards {
                if card_counts.get(&c).is_none() {
                    card_counts.insert(c, 1);
                } else {
                    *card_counts.get_mut(&c).unwrap() += 1;
                }
            }

            let mut distinct_counts: Vec<u8> = vec![];
            for (_, count) in card_counts.iter() {
                distinct_counts.push(*count)
            }
            let hand_type = if distinct_counts.len() == 5 {
                HandType::HighCard
            } else if distinct_counts.len() == 1 {
                HandType::FiveOfAKind
            } else if distinct_counts.len() == 2 {
                if card_counts.values().any(|x| *x == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            } else if distinct_counts.len() == 3 {
                if card_counts.values().any(|x| *x == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            } else if distinct_counts.len() == 4 {
                HandType::OnePair
            } else {
                panic!("Unexpected distinct counts in hand");
            };

            Hand {
                hand_type,
                cards,
                bid: bid.parse().unwrap(),
            }
        })
        .collect();
    hands.sort();

    let sum = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) as i64 * hand.bid);

    println!("part1: {}", sum);

    // Part2:
    let mut hands: Vec<Hand2> = inputs
        .split('\n')
        .map(|line| {
            let (hand_str, bid) = line.split_once(' ').unwrap();

            let mut cards = [Card2::A; 5];
            for (i, c) in hand_str.chars().enumerate() {
                cards[i] = Card2::from(c);
            }

            let mut card_counts = HashMap::<Card2, u8>::new();
            for c in cards {
                if card_counts.get(&c).is_none() {
                    card_counts.insert(c, 1);
                } else {
                    *card_counts.get_mut(&c).unwrap() += 1;
                }
            }

            let mut distinct_counts: Vec<u8> = vec![];
            for (_, count) in card_counts.iter() {
                distinct_counts.push(*count)
            }
            // Change this logic to take into account Js..
            // I hecking love if statements.
            let hand_type = if distinct_counts.len() == 5 {
                // Num J can only be 1
                if card_counts.get(&Card2::J).is_some() {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            } else if distinct_counts.len() == 1 {
                HandType::FiveOfAKind
            } else if distinct_counts.len() == 2 {
                if card_counts.values().any(|x| *x == 4) {
                    if let Some(js) = card_counts.get(&Card2::J) {
                        if *js == 1 || *js == 4 {
                            HandType::FiveOfAKind
                        } else {
                            panic!("unexpected Js");
                        }
                    } else {
                        HandType::FourOfAKind
                    }
                } else if let Some(js) = card_counts.get(&Card2::J) {
                    if *js == 2 || *js == 3 {
                        HandType::FiveOfAKind
                    } else {
                        panic!("unexpected Js");
                    }
                } else {
                    HandType::FullHouse
                }
            } else if distinct_counts.len() == 3 {
                // 3 1 1
                if card_counts.values().any(|x| *x == 3) {
                    if let Some(js) = card_counts.get(&Card2::J) {
                        if *js == 3 || *js == 1 {
                            HandType::FourOfAKind
                        } else {
                            panic!("Unexpxected Js");
                        }
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else {
                    // 2 2 1.
                    if let Some(js) = card_counts.get(&Card2::J) {
                        if *js == 2 {
                            HandType::FourOfAKind
                        } else if *js == 1 {
                            HandType::FullHouse
                        } else {
                            panic!("Unexpxected Js");
                        }
                    } else {
                        HandType::TwoPair
                    }
                }
            } else if distinct_counts.len() == 4 {
                // Num J can be 2 or 1
                if let Some(js) = card_counts.get(&Card2::J) {
                    if *js == 2 || *js == 1 {
                        HandType::ThreeOfAKind
                    } else {
                        panic!("unexpected Js");
                    }
                } else {
                    HandType::OnePair
                }
            } else {
                panic!("Unexpected distinct counts in hand");
            };

            Hand2 {
                hand_type,
                cards,
                bid: bid.parse().unwrap(),
            }
        })
        .collect();
    hands.sort();

    let sum = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) as i64 * hand.bid);

    println!("part2: {}", sum)
}

#[derive(Eq, Debug, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                let mut cmp_result = std::cmp::Ordering::Equal;
                for pair in self.cards.iter().zip(other.cards) {
                    match pair.0.cmp(&pair.1) {
                        std::cmp::Ordering::Less => {
                            cmp_result = std::cmp::Ordering::Less;
                            break;
                        }
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => {
                            cmp_result = std::cmp::Ordering::Greater;
                            break;
                        }
                    }
                }
                cmp_result
            }
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let hand_type_same = self.hand_type == other.hand_type;
        let mut cards_same = true;
        for pair in self.cards.iter().zip(other.cards) {
            if *pair.0 != pair.1 {
                cards_same = false;
                break;
            }
        }

        hand_type_same && cards_same
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Unexpeceted Card"),
        }
    }
}

#[derive(Eq, Debug, Clone)]
struct Hand2 {
    cards: [Card2; 5],
    bid: i64,
    hand_type: HandType,
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                let mut cmp_result = std::cmp::Ordering::Equal;
                for pair in self.cards.iter().zip(other.cards) {
                    match pair.0.cmp(&pair.1) {
                        std::cmp::Ordering::Less => {
                            cmp_result = std::cmp::Ordering::Less;
                            break;
                        }
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => {
                            cmp_result = std::cmp::Ordering::Greater;
                            break;
                        }
                    }
                }
                cmp_result
            }
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        let hand_type_same = self.hand_type == other.hand_type;
        let mut cards_same = true;
        for pair in self.cards.iter().zip(other.cards) {
            if *pair.0 != pair.1 {
                cards_same = false;
                break;
            }
        }

        hand_type_same && cards_same
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card2 {
    // Jokers are now considered the lowest strength card
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Q,
    K,
    A,
}

impl From<char> for Card2 {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Unexpeceted Card"),
        }
    }
}
