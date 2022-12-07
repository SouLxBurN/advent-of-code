use itertools::Itertools;

#[derive(Debug)]
struct Range {
    lb: u32,
    ub: u32,
}

impl Range {
    pub fn contains(left: &Range, right: &Range) -> bool {
        if left.lb <= right.lb && left.ub >= right.ub {
            // Left range contains Right Range.
            return true;
        }
        if right.lb <= left.lb && right.ub >= left.ub {
            // Right range contains Left Range.
            return true;
        }

        false
    }

    // (StartA <= EndB) and (EndA >= StartB)
    pub fn overlap(left: &Range, right: &Range) -> bool {
        left.lb <= right.ub && left.ub >= right.lb
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day4.input").unwrap();
    let lines = input.split("\n");

    let ranges: Vec<(Range,Range)> = lines.map_while(|line| {
        if line.len() > 0 {
            let ranges: (Range,Range) = line.split(",").map(|range| {
                let mut split = range.split("-");
                Range{
                    lb: split.next().unwrap().parse::<u32>().unwrap(),
                    ub: split.next().unwrap().parse::<u32>().unwrap()
                }
            }).collect_tuple().unwrap();
            Some(ranges)
        } else {
            None
        }
    }).collect();

    part1(&ranges);
    part2(&ranges);
}

fn part1(input: &Vec<(Range,Range)>) {
    let contains_count: u32 = input.iter()
        .fold(0u32, |mut acc, (left_range, right_range)| {
            if Range::contains(left_range,right_range) {
                acc += 1;
            }
            acc
    });
    println!("Part 1: {}", contains_count);
}

fn part2(input: &Vec<(Range,Range)>) {
    let overlap_count: u32 = input.iter()
        .fold(0u32, |mut acc, (left_range, right_range)| {
            if Range::overlap(left_range,right_range) {
                acc += 1;
            }
            acc
    });
    println!("Part 2: {}", overlap_count);
}
