use regex::Regex;

// Monkey 0:
//  Starting items: 59, 74, 65, 86
//  Operation: new = old * 19
//  Test: divisible by 7
//    If true: throw to monkey 6
//    If false: throw to monkey 2
#[derive(Debug, Clone)]
struct Monkey {
    _id: usize,
    items: Vec<u64>,
    op: Operation,
    test: MonkeyTest,
    inspections: u64
}

#[derive(Debug, Clone)]
enum Operation {
    ADD(u64),
    MUL(u64),
    DOUBLE,
    SQUARE
}

#[derive(Debug, Clone)]
struct MonkeyTest {
    div: u64,
    t: usize,
    f: usize
}

fn main() {
    part1(&mut parse_input());
    part2(&mut parse_input());
}


fn part1(monkeys: &mut Vec<Monkey>) {
    // 20 rounds
    (0..20).for_each(|_| {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m].clone();
            monkey.items.iter().for_each(|item| {
                let new_worry = match monkey.op {
                    Operation::ADD(v) => (item + v) / 3,
                    Operation::MUL(v) => (item * v) / 3,
                    Operation::DOUBLE => (item + item) / 3, // Not in input xD
                    Operation::SQUARE => (item * item) / 3,
                };

                if (new_worry % monkey.test.div) == 0 {
                    monkeys[monkey.test.t].items.push(new_worry);
                    // println!("I: {item}, op:{:?}, worry: {new_worry}, sent: {}", monkey.op, monkey.test.t);
                } else {
                    monkeys[monkey.test.f].items.push(new_worry);
                    // println!("I: {item}, op:{:?}, worry: {new_worry}, sent: {}", monkey.op, monkey.test.f);
                }
                monkeys[m].inspections += 1;
            });
            monkeys[m].items.clear();
        }
    });
    // println!("{:?}", monkeys);
    monkeys.sort_by_key(|monkey| monkey.inspections);
    println!("Part 1: {}", monkeys[monkeys.len()-2].inspections * monkeys[monkeys.len()-1].inspections);
}

fn part2(monkeys: &mut Vec<Monkey>) {
    // Big
    let big_prime = monkeys.iter().fold(1, |mut big_prime, mk| {
        big_prime *= mk.test.div;
        big_prime
    });

    // 10000 rounds
    (0..10000).for_each(|_| {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m].clone();
            monkey.items.iter().for_each(|item| {
                let new_worry = match monkey.op {
                    Operation::ADD(v) => (item + v) % big_prime,
                    Operation::MUL(v) => (item * v) % big_prime,
                    Operation::DOUBLE => (item + item) % big_prime, // Not in input xD
                    Operation::SQUARE => (item * item) % big_prime
                };

                if (new_worry % monkey.test.div) == 0 {
                    monkeys[monkey.test.t].items.push(new_worry);
                } else {
                    monkeys[monkey.test.f].items.push(new_worry);
                };
                monkeys[m].inspections += 1;
            });
            monkeys[m].items.clear();
        }
    });
    // println!("{:?}", monkeys);
    monkeys.sort_by_key(|monkey| monkey.inspections);
    println!("Part 2: {}", monkeys[monkeys.len()-2].inspections * monkeys[monkeys.len()-1].inspections);
}

fn parse_input() -> Vec<Monkey> {
    let input = std::fs::read_to_string("./src/bin/day11.input").unwrap();
    let int_only = Regex::new(r#"\D"#).unwrap();
    let parse_op = Regex::new(r#"^.*old\s(\+|\*)\s(\d+|\w+)$"#).unwrap();

    input.trim().split("\n\n").map(|monkey_str| {
        let mut m_iter = monkey_str.trim().split("\n");

        // Monkey ID
        let _id = int_only.replace_all(m_iter.next().unwrap(), "").parse::<usize>().unwrap();

        // Starting Items
        let items = m_iter.next().unwrap()
            .split(":").nth(1).unwrap().trim()
            .split(",").flat_map(|item| {
                item.trim().parse::<u64>()
        }).collect::<Vec<u64>>();

        // Operation
        let op_caps = parse_op.captures(m_iter.next().unwrap()).unwrap();
        let op = match op_caps.get(1).unwrap().as_str() {
            "*" => {
                match op_caps.get(2).unwrap().as_str() {
                    "old" => Operation::SQUARE,
                    v => Operation::MUL(v.parse::<u64>().unwrap())
                }
            },
            "+" => {
                match op_caps.get(2).unwrap().as_str() {
                    "old" => Operation::DOUBLE,
                    v => Operation::ADD(v.parse::<u64>().unwrap())
                }
            },
            _ => panic!(),
        };

        // Test
        let test = MonkeyTest{
            div: int_only.replace_all(m_iter.next().unwrap(), "").parse::<u64>().unwrap(),
            t: int_only.replace_all(m_iter.next().unwrap(), "").parse::<usize>().unwrap(),
            f: int_only.replace_all(m_iter.next().unwrap(), "").parse::<usize>().unwrap(),
        };

        Monkey{_id, items, op, test, inspections: 0}
    }).collect()
}
