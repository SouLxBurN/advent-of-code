fn main() {
    let input = std::fs::read_to_string("./src/bin/day5.input").expect("Input file not found");

    println!("part 1: {}", part1(input.clone()));
    println!("part 2: {}", part2_2(input.clone()));
}

fn part1(input: String) -> u64 {
    let mut chunks = input.split("\n\n");
    let seeds: Vec<u64> = chunks.next().unwrap()
        .split(" ")
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let maps: Vec<RangeMap> = chunks.map(parse_map).collect();

    seeds.iter().map(|s| {
        let mut seed = *s;
        for map in &maps {
            seed = map.map_source(seed);
        }
        println!("seed: {} -> location: {}", s, seed);
        seed
    }).min().unwrap()
}

fn part2_2(input: String) -> u64 {
    let mut chunks = input.split("\n\n");
    let seed_ranges: Vec<SeedRange> = parse_seeds(chunks.next().unwrap());
    let maps: Vec<RangeMap> = chunks.map(parse_map).collect();

    seed_ranges.iter().flat_map(|sr| {
        sr.lower..sr.upper
    }).map(|s| {
        let mut seed = s;
        for map in &maps {
            seed = map.map_source(seed);
        }
        seed
    }).min().unwrap()
}

// 52 50 48
// swap (source)50 -> (destination)52 (range)48
// source: 50 -> 50 + (48-1) = 97
// destination shift: (destination)52 - (source)50 = 2
// example: seed 68: 68 + = 70

#[derive(Debug)]
struct RangeMap {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    source_lower: u64,
    source_upper: u64,
    dest_lower: u64,
    dest_upper: u64,
    shift: i64,
    range: u64,
}

#[derive(Debug, Clone)]
struct SeedRange {
    lower: u64,
    upper: u64,
}

impl SeedRange {
    fn verify(&self) -> bool {
        self.lower <= self.upper
    }
}

impl RangeMap {
    fn verify(&self) {
        for range in &self.ranges {
            assert!(range.source_lower <= range.source_upper);
            assert!(range.dest_lower <= range.dest_upper);
            assert!(range.source_upper.checked_add_signed(range.shift).unwrap() == range.dest_upper);
            assert!(range.range == range.source_upper - range.source_lower + 1);
            assert!(range.range == range.dest_upper - range.dest_lower + 1);
        }
    }

    fn map_source(&self, input: u64) -> u64 {
        for range in &self.ranges {
            if input >= range.source_lower && input <= range.source_upper {
                return input.checked_add_signed(range.shift).unwrap();
            }
        }
        input
    }
    // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    // [  A ][     B    ][     C    ]
    // Where Range [2 - 5]
    fn map_seed_range_source(&self, input: &SeedRange) -> Vec<SeedRange> {
        let mut ranges = Vec::new();
        for range in &self.ranges {
            // A
            if input.lower < range.source_lower {
                if input.upper >= range.source_lower {
                    ranges.push(SeedRange{
                        lower: input.lower,
                        upper: range.source_lower - 1,
                    });
                    if !ranges.last().unwrap().verify() {
                        println!("B: {:?} -> {:?}", input, range);
                        println!("B: {:?}", ranges.last().unwrap());
                        panic!();
                    }
                } else {
                    ranges.push(input.clone());
                    if !ranges.last().unwrap().verify() {
                        println!("B: {:?} -> {:?}", input, range);
                        println!("B: {:?}", ranges.last().unwrap());
                        panic!();
                    }
                }
            }

            // B
            if input.lower >= range.source_lower {
                if input.upper <= range.source_upper {
                    ranges.push(SeedRange{
                        lower: input.lower.checked_add_signed(range.shift).unwrap(),
                        upper: input.upper.checked_add_signed(range.shift).unwrap(),
                    });
                    if !ranges.last().unwrap().verify() {
                        println!("B: {:?} -> {:?}", input, range);
                        println!("B: {:?}", ranges.last().unwrap());
                        panic!();
                    }
                } else if input.lower <= range.source_upper{
                    ranges.push(SeedRange{
                        lower: input.lower.checked_add_signed(range.shift).unwrap(),
                        upper: range.dest_upper,
                    });
                    if !ranges.last().unwrap().verify() {
                        println!("B: {:?} -> {:?}", input, range);
                        println!("B: {:?}", ranges.last().unwrap());
                        panic!();
                    }
                }
            } else {
                if input.upper >= range.source_lower {
                    if input.upper <= range.source_upper {
                        ranges.push(SeedRange{
                            lower: range.dest_lower,
                            upper: input.upper.checked_add_signed(range.shift).unwrap(),
                        });
                        if !ranges.last().unwrap().verify() {
                            println!("B: {:?} -> {:?}", input, range);
                            println!("B: {:?}", ranges.last().unwrap());
                            panic!();
                        }
                    } else {
                        ranges.push(SeedRange{
                            lower: range.dest_lower,
                            upper: range.dest_upper,
                        });
                        if !ranges.last().unwrap().verify() {
                            println!("B: {:?} -> {:?}", input, range);
                            println!("B: {:?}", ranges.last().unwrap());
                            panic!();
                        }
                    }
                }
            }

            // C
            // [4, 5, 6]
            //          [7, 8, 9]
            if input.upper > range.source_upper {
                if input.lower <= range.source_upper {
                    ranges.push(SeedRange{
                        lower: range.source_upper + 1,
                        upper: input.upper,
                    });
                    if !ranges.last().unwrap().verify() {
                        println!("B: {:?} -> {:?}", input, range);
                        println!("B: {:?}", ranges.last().unwrap());
                        panic!();
                    }
                } else {
                    ranges.push(input.clone());
                    if !ranges.last().unwrap().verify() {
                        println!("B: {:?} -> {:?}", input, range);
                        println!("B: {:?}", ranges.last().unwrap());
                        panic!();
                    }
                }
            }
        }
        ranges
    }
}

fn parse_seeds(input: &str) -> Vec<SeedRange> {
    let mut seeds = Vec::new();
    let mut seed_iter = input.split(" ").skip(1);
    while let Some(seed_start) = seed_iter.next() {
        let start = seed_start.parse::<u64>().unwrap();
        let range = seed_iter.next().unwrap().parse::<u64>().unwrap();

        seeds.push(SeedRange{
            lower: start,
            upper: start + range - 1,
        });
    }
    seeds
}

fn parse_map(input: &str) -> RangeMap {
    let ranges = input
        .lines()
        .skip(1)
        .map(|line| {
            let mut parts = line.split(" ");
            let destination = parts.next().unwrap().parse::<u64>().unwrap();
            let source = parts.next().unwrap().parse::<u64>().unwrap();
            let range = parts.next().unwrap().parse::<u64>().unwrap();

            Range {
                source_lower: source,
                source_upper: source + (range - 1),
                dest_lower: destination,
                dest_upper: destination + (range - 1),
                shift: destination as i64 - source as i64,
                range,
            }
        })
        .collect();

    RangeMap { ranges }
}

fn part2(input: String) -> u64 {
    let mut chunks = input.split("\n\n");
    let mut seeds = parse_seeds(chunks.next().unwrap());
    let maps: Vec<RangeMap> = chunks.map(parse_map).collect();
    maps.iter().for_each(|map| {
        map.verify();
    });

    for map in &maps {
        seeds = seeds.iter().map(|sr| {
            let ranges = map.map_seed_range_source(sr);
            ranges
        }).flatten().collect();
    }
    seeds.iter().fold(u64::MAX, |acc, sr| {
        if sr.lower < acc {
            return sr.lower;
        }
        acc
    })
}
