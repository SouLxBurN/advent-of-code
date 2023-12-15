fn main() {
    let input = std::fs::read_to_string("./src/bin/day6.input").expect("Input file not found");

    println!("part 1: {}", part1(input.clone()));
    println!("part 2: {}", part2(input.clone()));
}

fn calculate_options(time: f64, distance: f64) -> i64 {
    // button_time * (total_time - button_time) = distance
    // x * (7 - x) - 9 = 0
    // -x^2 + 7x - 9 = 0
    // x = (-b +- sqrt(b^2 - 4ac)) / 2a
    let x1 = (-time + (time.powi(2) - (-4f64 * -distance)).sqrt()) / -2f64;
    let x2 = (-time - (time.powi(2) - (-4f64 * -distance)).sqrt()) / -2f64;

    let mut options = (x2.floor() - x1.floor()) as i64;
    if x2.floor() == x2 {
        options -= 1;
    }
    // println!("x1: {}, x2: {}, options: {}", x1, x2, options);
    options
}

fn part1(input: String) -> i64 {
    let time_distance_input: Vec<Vec<f64>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(str::parse::<f64>)
                .filter_map(Result::ok)
                .collect()
        })
        .collect();

    let time = &time_distance_input[0];
    let distance = &time_distance_input[1];
    (0..time_distance_input[0].len())
        .fold(1i64, |acc, i| acc * calculate_options(time[i], distance[i]))
}

fn part2(input: String) -> i64 {
    let time_distance_input: Vec<f64> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|x| !x.is_ascii_whitespace())
                .collect::<String>()
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .parse::<f64>()
                .unwrap()
        })
        .collect();

    calculate_options(time_distance_input[0] as f64, time_distance_input[1] as f64) as i64
}
