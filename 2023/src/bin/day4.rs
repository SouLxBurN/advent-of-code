use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./src/bin/day4.input").expect("Input file not found");

    println!("part 1: {}", part1(input.clone()));
    println!("Let part 2 cook");
    println!("part 2: {}", part2(input.clone()));
}

#[derive(Debug)]
struct Card {
    id: usize,
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl Card {
    fn calculate_points(&self) -> u32 {
        self.numbers.iter().fold(0, |total, num| {
            if self.winning_numbers.contains(num) {
                if total == 0 {
                    return 1;
                } else {
                    return total * 2;
                }
            }
            total
        })
    }

    fn calculate_matches(&self) -> u32 {
        self.numbers.iter().fold(0, |total, num| {
            if self.winning_numbers.contains(num) {
                return total + 1;
            }
            total
        })
    }
}

fn parse_cards(input: String) -> Vec<Card> {
    input
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            let numbers = line.split(":").collect::<Vec<&str>>()[1]
                .trim()
                .split("|")
                .collect::<Vec<&str>>();

            let winning_numbers = numbers[0]
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let card_numbers = numbers[1]
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            Card {
                id: line_num + 1,
                numbers: card_numbers,
                winning_numbers,
            }
        })
        .collect()
}

fn part1(input: String) -> u32 {
    let cards = parse_cards(input);
    cards
        .iter()
        .map(|card| card.calculate_points())
        .sum::<u32>()
}

fn part2(input: String) -> u32 {
    let cards = parse_cards(input);
    let mut total = 0;
    let mut card_queue: Vec<usize> = Vec::new();

    let card_map = cards
        .into_iter()
        .map(|card| {
            let mut matches = card.calculate_matches();
            total += 1;

            let mut new_card_id = card.id;
            while matches > 0 {
                new_card_id = new_card_id+1;
                card_queue.push(new_card_id);
                matches -= 1;
            }
            (card.id, card)
        })
        .collect::<HashMap<usize, Card>>();

    while let Some(card_id) = card_queue.pop() {
        let card = card_map.get(&card_id).unwrap();
        let mut matches = card.calculate_matches();
        total += 1;

        let mut new_card_id = card.id;
        while matches > 0 {
            new_card_id = new_card_id+1;
            card_queue.push(new_card_id);
            matches -= 1;
        }
    }
    total
}
