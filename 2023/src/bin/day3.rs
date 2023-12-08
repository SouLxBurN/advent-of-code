use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./src/bin/day3.input").expect("Input file not found");
    println!("part 1: {}", part1(input.clone()));
    println!("part 2: {}", part2(input.clone()));
}

fn part1(input: String) -> u32 {
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut matched_idx: Vec<(usize,usize)> = Vec::new();
    let mut total: u32 = 0;

    schematic.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, col)| {
            if !col.is_digit(10) && col.ne(&'.') {
                // [-1, -1], [-1, 0], [-1, 1]
                // [ 0, -1], [-----], [ 0, 1]
                // [ 1, -1], [ 1, 0], [ 1, 1]
                vec![
                    (-1i32, -1i32),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .into_iter()
                .for_each(|(y, x)| {
                    let row_chx = (y + row_idx as i32) as usize;
                    let col_chx = (x + col_idx as i32) as usize;

                    if schematic[row_chx][col_chx].is_digit(10)
                        && schematic[row_chx][col_chx].ne(&'.')
                    {
                        // Crawl the columns until reaching the start of the number.
                        let mut left_idx = col_chx;
                        while left_idx > 0 && schematic[row_chx][left_idx - 1].is_digit(10) {
                            left_idx -= 1;
                        }

                        // Crawl the columns until reaching the end of the number.
                        let mut right_idx = left_idx;
                        while right_idx < schematic[row_chx].len()
                            && schematic[row_chx][right_idx].is_digit(10)
                        {
                            right_idx += 1;
                        }

                        let number = schematic[row_chx][left_idx..right_idx]
                            .into_iter()
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap();

                        if !matched_idx.contains(&(left_idx, row_chx)) {
                            matched_idx.push((left_idx, row_chx));
                            total += number;
                        }
                    }
                });
            }
        });
    });
    total
}

fn part2(input: String) -> u32 {
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut matched_idx: Vec<(usize,usize)> = Vec::new();
    let mut gear_ratios: HashMap<(usize,usize), Vec<u32>> = HashMap::new();

    schematic.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, col)| {
            if !col.is_digit(10) && col.eq(&'*') {
                // [-1, -1], [-1, 0], [-1, 1]
                // [ 0, -1], [-----], [ 0, 1]
                // [ 1, -1], [ 1, 0], [ 1, 1]
                vec![
                    (-1i32, -1i32),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .into_iter()
                .for_each(|(y, x)| {
                    let row_chx = (y + row_idx as i32) as usize;
                    let col_chx = (x + col_idx as i32) as usize;

                    if schematic[row_chx][col_chx].is_digit(10)
                        && schematic[row_chx][col_chx].ne(&'.')
                    {
                        // Crawl the columns until reaching the start of the number.
                        let mut left_idx = col_chx;
                        while left_idx > 0 && schematic[row_chx][left_idx - 1].is_digit(10) {
                            left_idx -= 1;
                        }

                        // Crawl the columns until reaching the end of the number.
                        let mut right_idx = left_idx;
                        while right_idx < schematic[row_chx].len()
                            && schematic[row_chx][right_idx].is_digit(10)
                        {
                            right_idx += 1;
                        }

                        let number = schematic[row_chx][left_idx..right_idx]
                            .into_iter()
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap();

                        if !matched_idx.contains(&(left_idx, row_chx)) {
                            matched_idx.push((left_idx, row_chx));
                            if !gear_ratios.contains_key(&(row_idx, col_idx)) {
                                gear_ratios.insert((row_idx, col_idx), Vec::new());
                            }
                            gear_ratios.get_mut(&(row_idx, col_idx)).unwrap().push(number);
                        }
                    }
                });
            }
        });
    });

    gear_ratios.values().fold(0u32, |mut acc, numbers| {
        if numbers.len() == 2 {
            acc += numbers[0] * numbers[1]
        }
        acc
    })
}
