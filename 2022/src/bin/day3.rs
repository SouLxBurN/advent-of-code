use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("./src/bin/day3.input").unwrap();
    let priority_map = init_priority_map();

    part1(&input, &priority_map);
    part2(&input, &priority_map);
}

fn part1(input: &String, priority_map: &HashMap<char, u32>) {
    let sum_priorities: u32 = input.split("\n").filter_map(|line| {
        if line.len() <= 1 {
            return None;
        }

        let (cont_l, cont_r) = line.split_at(line.len() / 2);

        let seen: HashSet<char> = cont_l.char_indices().fold(HashSet::new(), |mut seen, c| {
            seen.insert(c.1);
            seen
        });

        let pair = cont_r.char_indices().find(|c| {
            seen.get(&c.1).is_some()
        }).unwrap().1;

        Some(pair)
    }).fold(0u32, |acc, c| acc + priority_map.get(&c).unwrap());

    println!("Part 1: {}", sum_priorities);
}

fn part2(input: &String, priority_map: &HashMap<char, u32>) {
    let elf_group_size = 3;
    let split: Vec<&str> = input.split("\n").collect();

    let sum_priorities = split.chunks(elf_group_size).filter_map(|lines| {
        if lines.len() <= elf_group_size-1 {
            return None;
        }
        let mut seen: Vec<HashSet<char>> = vec![HashSet::new(); lines.len()-1];
        let triplet = lines.into_iter().enumerate().fold(None, |acc, (idx, line)| {
            if acc.is_some() {
                return acc;
            }
            line.char_indices().find(|c| {
                if idx == elf_group_size-1 && seen.iter().fold(true, |acc, map| acc && map.get(&c.1).is_some()) {
                    return true;
                } else if idx < elf_group_size-1 {
                    seen[idx].insert(c.1);
                }
                false
            })
        }).unwrap().1;

        Some(triplet)
    }).fold(0u32, |acc, c| acc + priority_map.get(&c).unwrap());

    println!("Part 2: {}", sum_priorities);
}

fn init_priority_map() -> HashMap<char, u32> {
    let mut map: HashMap<char, u32> = HashMap::new();

    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);
    map.insert('d', 4);
    map.insert('e', 5);
    map.insert('f', 6);
    map.insert('g', 7);
    map.insert('h', 8);
    map.insert('i', 9);
    map.insert('j', 10);
    map.insert('k', 11);
    map.insert('l', 12);
    map.insert('m', 13);
    map.insert('n', 14);
    map.insert('o', 15);
    map.insert('p', 16);
    map.insert('q', 17);
    map.insert('r', 18);
    map.insert('s', 19);
    map.insert('t', 20);
    map.insert('u', 21);
    map.insert('v', 22);
    map.insert('w', 23);
    map.insert('x', 24);
    map.insert('y', 25);
    map.insert('z', 26);
    map.insert('A', 27);
    map.insert('B', 28);
    map.insert('C', 29);
    map.insert('D', 30);
    map.insert('E', 31);
    map.insert('F', 32);
    map.insert('G', 33);
    map.insert('H', 34);
    map.insert('I', 35);
    map.insert('J', 36);
    map.insert('K', 37);
    map.insert('L', 38);
    map.insert('M', 39);
    map.insert('N', 40);
    map.insert('O', 41);
    map.insert('P', 42);
    map.insert('Q', 43);
    map.insert('R', 44);
    map.insert('S', 45);
    map.insert('T', 46);
    map.insert('U', 47);
    map.insert('V', 48);
    map.insert('W', 49);
    map.insert('X', 50);
    map.insert('Y', 51);
    map.insert('Z', 52);

    map
}

