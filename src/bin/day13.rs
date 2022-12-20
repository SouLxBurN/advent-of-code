use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Self>),
    DIGIT(u32),
}

impl Packet {
    fn push(&mut self, p: Packet) {
        match self {
            Self::List(list) => list.push(p),
            Self::DIGIT(_) => panic!("Can't push here!"),
        }
    }

    fn get_list(&self) -> Option<&Vec<Self>> {
        match self {
            Self::List(l) => Some(l),
            Self::DIGIT(_) => None
        }
    }
}

fn parse_packet_pairs(input: &str) -> Vec<PacketPair> {
    input.trim().split("\n\n")
        .map(|line_pair| {
            let mut pair_iter = line_pair.trim().split("\n");
            PacketPair {
                left: parse_packet(pair_iter.next().unwrap()),
                right: parse_packet(pair_iter.next().unwrap()),
            }
        })
        .collect()
}

fn parse_packet(input: &str) -> Packet {
    let mut packet_stack: Vec<Packet> = vec!();
    let mut parent = Packet::DIGIT(0);
    let chars = &mut input.trim().chars();

    let mut collect_digits: Vec<char> = vec!();
    while let Some(c) = chars.next() {
        match c {
            '[' => {
                let new_packet = Packet::List(vec![]);
                packet_stack.push(new_packet);
            },
            ']' => {
                let mut tmp = packet_stack.pop().unwrap();
                if collect_digits.len() > 0 {
                    tmp.push(Packet::DIGIT(char_vec_to_decimal(&mut collect_digits)));
                }
                if let Some(p) = packet_stack.last_mut() {
                    p.push(tmp);
                } else {
                    parent = tmp
                }
            },
            ',' => {
                if collect_digits.len() > 0 {
                    packet_stack.last_mut().expect("Shouldn't be empty")
                        .push(Packet::DIGIT(char_vec_to_decimal(&mut collect_digits)));
                }
            },
            _d => {
                collect_digits.push(_d);
            }
        };
    }
    parent
}

fn char_vec_to_decimal(digits: &mut Vec<char>) -> u32 {
    let mut place = 0;
    let mut value = 0;
    while let Some(d) = digits.pop() {
        value += d.to_digit(10).unwrap() * 10u32.pow(place);
        place += 1;
    }
    value
}

enum CResult {
    Valid,
    NotValid,
    Continue
}

fn compare_lists(left: &Vec<Packet>, right: &Vec<Packet>) -> CResult {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    let mut l_op = left_iter.next();
    let mut r_op = right_iter.next();
    while l_op.is_some() && r_op.is_some() {
        match l_op.unwrap() {
            Packet::List(left_list) => {
                match r_op.unwrap() {
                    Packet::List(right_list) => {
                        // Left and Right are lists
                        let result = compare_lists(left_list, right_list);
                        match result {
                            CResult::Valid | CResult::NotValid => return result,
                            CResult::Continue => {}
                        }
                    },
                    Packet::DIGIT(right_digit) => {
                        // Left is a List, Right is a Digit
                        let right_list = vec!(Packet::DIGIT(*right_digit));
                        let result = compare_lists(left_list, &right_list);
                        match result {
                            CResult::Valid | CResult::NotValid => return result,
                            CResult::Continue => {}
                        }
                    }
                }
            },
            Packet::DIGIT(left_digit) => {
                match r_op.unwrap() {
                    Packet::List(right_list) => {
                        // Left is a Digit, Right is a List
                        let left_list = vec!(Packet::DIGIT(*left_digit));
                        let result = compare_lists(&left_list, right_list);
                        match result {
                            CResult::Valid | CResult::NotValid => return result,
                            CResult::Continue => {}
                        }
                    },
                    Packet::DIGIT(right_digit) => {
                        // Left and Right are Digits
                        if left_digit < right_digit {
                            return CResult::Valid;
                        } else if left_digit > right_digit {
                            return CResult::NotValid;
                        }
                    }
                }
            }
        }
        l_op = left_iter.next();
        r_op = right_iter.next();
    }

    if l_op.is_some() && r_op.is_none() {
        // Right ran out of items
        return CResult::NotValid;
    } else if l_op.is_none() && r_op.is_some() {
        // Left ran out of items
        return CResult::Valid;
    }

    CResult::Continue
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day13.input").unwrap();
    let mut pairs = parse_packet_pairs(&input);

    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&mut pairs));
}

fn part1(pairs: &Vec<PacketPair>) -> usize {
    let valid_count = pairs.iter().enumerate().fold(0, |mut valid_cnt, (index, pair)| {
        let left = pair.left.get_list().unwrap();
        let right = pair.right.get_list().unwrap();
        match compare_lists(&left, &right) {
            CResult::Valid | CResult::Continue => valid_cnt += index+1,
            CResult::NotValid => {},
        }
        valid_cnt
    });
    valid_count
}

fn part2(pairs: &mut Vec<PacketPair>) -> usize {
    let dividers = PacketPair {
        left: Packet::List(vec!(Packet::List(vec!(Packet::DIGIT(2))))),
        right: Packet::List(vec!(Packet::List(vec!(Packet::DIGIT(6))))),
    };
    pairs.push(dividers.clone());

    let mut flat_packets: Vec<&Packet> = pairs.iter().flat_map(|pair| {
        vec!(&pair.left, &pair.right)
    }).collect();

    flat_packets.sort_by(|left, right| -> Ordering {
        match compare_lists(left.get_list().unwrap(), right.get_list().unwrap()) {
            CResult::Valid => Ordering::Less,
            CResult::NotValid => Ordering::Greater,
            CResult::Continue => Ordering::Equal,
        }
    });

    flat_packets.iter().enumerate().fold(0, |mut decoder_key, (index, packet)| {
        if **packet == dividers.left {
            decoder_key = index+1;
        }
        if **packet == dividers.right {
            decoder_key *= index+1;
        }
        decoder_key
    })
}

