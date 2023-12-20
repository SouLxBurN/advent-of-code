use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./src/bin/day8.input").expect("Input file not found");

    println!("part 1: {}", part1(input.clone()));
    println!("part 2: {}", part2(input.clone()));
}

#[derive(Debug)]
enum Step {
    Right,
    Left,
}

#[derive(Debug)]
struct Node {
    value: String,
    left: String,
    right: String,
}

fn parse_steps(steps: &str) -> Vec<Step> {
    steps
        .chars()
        .map(|c| match c {
            'R' => Step::Right,
            'L' => Step::Left,
            _ => panic!("Invalid step"),
        })
        .collect()
}

fn parse_nodes(nodes: &str) -> (Vec<&str>, HashMap<&str, Node>) {
    let mut starters = Vec::new();
    let map: HashMap<&str, Node> = nodes
        .lines()
        .map(|line| {
            let mut split = line.split("=");
            let value = split.next().unwrap().trim();
            let mut intruction_split = split.next().unwrap().trim().split(",");
            let left = intruction_split
                .next()
                .unwrap()
                .trim()
                .strip_prefix("(")
                .expect("Bad Parsing '('")
                .to_string();
            let right = intruction_split
                .next()
                .unwrap()
                .trim()
                .strip_suffix(")")
                .expect("Bad Parsing ')'")
                .to_string();
            if value.ends_with("A") {
                starters.push(value);
            }

            (
                value,
                Node {
                    value: value.to_string(),
                    left,
                    right,
                },
            )
        })
        .collect();
    (starters, map)
}

fn part1(input: String) -> u64 {
    let mut split = input.split("\n\n");
    let steps = parse_steps(split.next().unwrap());
    let (_, nodes) = parse_nodes(split.next().unwrap());

    let mut current_node = nodes
        .get("AAA")
        .expect("But Puzzle Input AAA must be present");
    let mut current_step = 0;
    let mut step_count = 0;
    while current_node.value != "ZZZ" {
        let step = &steps[current_step];
        match step {
            Step::Right => current_node = nodes.get(current_node.right.as_str()).expect("Node not found"),
            Step::Left => current_node = nodes.get(current_node.left.as_str()).expect("Node not found"),
        }

        current_step = if current_step == steps.len() - 1 {
            0
        } else {
            current_step + 1
        };
        step_count += 1;
    }
    step_count
}

fn part2(input: String) -> u64 {
    let mut split = input.split("\n\n");
    let steps = parse_steps(split.next().unwrap());
    let (starters, nodes) = parse_nodes(split.next().unwrap());
    let current_nodes = starters.iter().map(|n| &nodes[n]).collect::<Vec<&Node>>();
    let move_list: Vec<u64> = current_nodes.iter().map(|n| {
        let mut current_node = *n;
        let mut current_step = 0;
        let mut step_count = 0;
        while !current_node.value.ends_with("Z") {
            let step = &steps[current_step];
            match step {
                Step::Right => current_node = nodes.get(current_node.right.as_str()).expect("Node not found"),
                Step::Left => current_node = nodes.get(current_node.left.as_str()).expect("Node not found"),
            }

            current_step = if current_step == steps.len() - 1 {
                0
            } else {
                    current_step + 1
                };
            step_count += 1;
        }
        step_count
    }).collect();

    // Find the LCM of the move list
    // Could be done faster with the Euclidean Algorithm
    // This was fast enough for me though
    let max = *move_list.iter().max().unwrap();
    let mut lcm = max;
    while move_list.iter().any(|x| lcm % x != 0) {
        lcm += max;
    };
    lcm
}
