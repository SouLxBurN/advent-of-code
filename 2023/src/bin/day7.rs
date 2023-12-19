fn main() {
    let input = std::fs::read_to_string("./src/bin/day7.input").expect("Input file not found");

    println!("part 1: {}", 251121738);
    println!("part 2: {}", execute(input.clone()));
}

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Ord)]
enum Card{
    Ace = 14,
    King = 13,
    Queen = 12,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'T' => Card::Ten,
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => Card::from(c.to_digit(10).unwrap()),
        }
    }
}

impl From<u32> for Card {
    fn from(i: u32) -> Self {
        match i {
            1 => Card::Joker,
            2 => Card::Two,
            3 => Card::Three,
            4 => Card::Four,
            5 => Card::Five,
            6 => Card::Six,
            7 => Card::Seven,
            8 => Card::Eight,
            9 => Card::Nine,
            10 => Card::Ten,
            12 => Card::Queen,
            13 => Card::King,
            14 => Card::Ace,
            _ => panic!("Invalid card value: {}", i),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type_ordering = self.hand_type.partial_cmp(&other.hand_type);
        if type_ordering.unwrap_or(std::cmp::Ordering::Equal) == std::cmp::Ordering::Equal {
            for i in 0..self.cards.len() {
                let card_ordering = self.cards[i].cmp(&other.cards[i]);
                if card_ordering != std::cmp::Ordering::Equal {
                    return Some(card_ordering);
                }
            }
            type_ordering
        } else {
            type_ordering
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_ordering = self.hand_type.cmp(&other.hand_type);
        if type_ordering == std::cmp::Ordering::Equal {
            for i in 0..self.cards.len() {
                let card_ordering = self.cards[i].cmp(&other.cards[i]);
                if card_ordering != std::cmp::Ordering::Equal {
                    return card_ordering;
                }
            }
            type_ordering
        } else {
            type_ordering
        }
    }
}

fn execute(input: String) -> u64 {
    let mut hands: Vec<Hand> = input.lines().map(|line| {
        let mut split = line.split(" ");
        let cards = split.next().unwrap().chars().map(|x| Card::from(x)).collect::<Vec<Card>>();
        let mut card_counts: Vec<(&Card, i64)> = cards.iter().fold(Vec::new(), |mut acc, card| {
            if acc.len() == 0 {
                acc.push((card, 1));
            } else {
                if let Some(c) = acc.iter_mut().find(|x| x.0 == card) {
                    c.1 += 1;
                } else {
                    acc.push((card, 1));
                }
            }
            acc
        });
        card_counts.sort_by(|a, b| {
            a.0.cmp(&b.0).reverse()
        });

        let hand_type = card_counts.iter().fold(HandType::HighCard, |acc, card| {
            match card {
                (Card::Joker, x) => {
                    match x {
                        5 => HandType::FiveOfAKind,
                        4 => HandType::FiveOfAKind,
                        3 => match acc {
                            HandType::OnePair => HandType::FiveOfAKind,
                            _ => HandType::FourOfAKind,
                        },
                        2 => match acc {
                                HandType::ThreeOfAKind => HandType::FiveOfAKind,
                                HandType::OnePair => HandType::FourOfAKind,
                                _ => HandType::ThreeOfAKind,
                            },
                        1 => match acc {
                            HandType::FourOfAKind => HandType::FiveOfAKind,
                            HandType::ThreeOfAKind => HandType::FourOfAKind,
                            HandType::TwoPair => HandType::FullHouse,
                            HandType::OnePair => HandType::ThreeOfAKind,
                            _ => HandType::OnePair,
                        }
                        _ => acc,
                    }
                },
                (_, x) => match x {
                    5 => HandType::FiveOfAKind,
                    4 => HandType::FourOfAKind,
                    3 => {
                        match acc {
                            HandType::TwoPair => HandType::FullHouse,
                            HandType::OnePair => HandType::FullHouse,
                            _ => HandType::ThreeOfAKind,
                        }
                    },
                    2 => {
                        match acc {
                            HandType::ThreeOfAKind => HandType::FullHouse,
                            HandType::OnePair => HandType::TwoPair,
                            _ => HandType::OnePair,
                        }
                    },
                    _ => acc,
                }
            }
        });

        Hand{
            cards,
            bid: split.next().unwrap().parse::<u64>().unwrap(),
            hand_type
        }
    }).collect();
    hands.sort();

    hands.iter().enumerate().fold(0, |mut acc, (i, hand)| {
        acc += hand.bid * (i as u64 + 1);
        acc
    })
}
