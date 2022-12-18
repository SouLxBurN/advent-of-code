use std::collections::{VecDeque, HashSet, HashMap};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
    elevation: u8,
}

impl Node {
    fn is_neighbor(&self, other: &Node, reverse: bool) -> bool {
        match reverse {
            true => other.elevation >= self.elevation-1,
            false => other.elevation <= self.elevation+1,
        }
    }
}

const END: &u8 = &69u8;
const START: &u8 = &83u8;
const LOWEST_ELEVATION: &u8 = &97;

fn main() {
    let (grid, start, end) = load_input();
    part1(&grid, start, end);
    part2(&grid, start, end);
}

fn part1(grid: &Vec<Vec<Node>>, start: (usize, usize), end: (usize, usize)) {
    let (parent_of, mut walk) = search(&grid, start, |n: &Node| -> bool {
        n.y == end.0 && n.x == end.1
    }, false);

    let mut number_of_steps = 0;
    while let Some(n) = parent_of.get(&walk.unwrap()) {
        // println!("({},{})", n.y, n.x);
        walk = Some(n);
        number_of_steps += 1;
    }
    println!("Part 1: {}", number_of_steps);

}

fn part2(grid: &Vec<Vec<Node>>, _start: (usize, usize), end: (usize, usize)) {
    let (parent_of, mut walk) = search(&grid, end, |n: &Node| -> bool {
        n.elevation == 0
    }, true);

    let mut number_of_steps = 0;
    while let Some(n) = parent_of.get(&walk.unwrap()) {
        // println!("({},{})", n.y, n.x);
        walk = Some(n);
        number_of_steps += 1;
    }

    println!("Part 2: {}", number_of_steps);
}

fn search<F>(grid: &Vec<Vec<Node>>, start: (usize,usize), is_end: F, reverse: bool) -> (HashMap<&Node,&Node>, Option<&Node>)
    where F: Fn(&Node) -> bool {

    let mut visited: HashSet<&Node> = HashSet::from([&grid[start.0][start.1]]);
    let mut node_queue = VecDeque::from([&grid[start.0][start.1]]);
    let mut parent_of: HashMap<&Node, &Node> = HashMap::new();

    while !node_queue.is_empty() {
        let n = node_queue.pop_front().unwrap();
        if is_end(n) {
            return (parent_of, Some(n));
        }
        let neighbors = get_neighbors(&grid, n, reverse);
        for neighbor in neighbors.iter() {
            if !visited.contains(neighbor) {
                node_queue.push_back(neighbor); // Queue Neighbor
                visited.insert(neighbor); // Mark Visited
                parent_of.insert(neighbor, n); // Store Parent Reference
            }
        }
    }
    (parent_of, None)
}

fn get_neighbors<'a>(grid: &'a Vec<Vec<Node>>, node: &'a Node, reverse: bool) -> Vec<&'a Node> {
    let mut neighbors = vec!();
    let y_max = grid.len()-1;
    let x_max = grid[0].len()-1;

    if node.y > 0 {
        let neighbor = &grid[node.y-1][node.x];
        if node.is_neighbor(neighbor, reverse) {
            neighbors.push(neighbor);
        }

    }
    if node.y < y_max {
        let neighbor = &grid[node.y+1][node.x];
        if node.is_neighbor(neighbor, reverse) {
            neighbors.push(neighbor);
        }
    }

    if node.x > 0 {
        let neighbor = &grid[node.y][node.x-1];
        if node.is_neighbor(neighbor, reverse) {
            neighbors.push(neighbor);
        }
    }
    if node.x < x_max {
        let neighbor = &grid[node.y][node.x+1];
        if node.is_neighbor(neighbor, reverse) {
            neighbors.push(neighbor);
        }
    }

    neighbors
}

fn load_input() -> (Vec<Vec<Node>>, (usize,usize), (usize, usize)) {
    let input = std::fs::read_to_string("./src/bin/day12.input").unwrap();
    let mut start = (0usize, 0usize);
    let mut end = (0usize, 0usize);

    let grid: Vec<Vec<Node>> = input.trim()
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes().iter()
                .enumerate()
                .map(|(x, elevation)| {
                    if elevation < LOWEST_ELEVATION {
                        match elevation {
                            elev if elev == END => {
                                end = (y, x);
                                Node{x, y, elevation: 25}
                            },
                            elev if elev == START => {
                                start = (y, x);
                                Node{x, y, elevation: 0}
                            },
                            _ => panic!("Bad Elevation / Start / End")
                        }
                    } else {
                        Node{x, y, elevation: elevation - LOWEST_ELEVATION}
                    }
                }).collect::<Vec<Node>>()
        }).collect();
    (grid, start, end)
}
