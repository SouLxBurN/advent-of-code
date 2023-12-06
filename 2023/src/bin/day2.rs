fn main() {
    let input = std::fs::read_to_string("./src/bin/day2.input").expect("Input file not found");
    println!("part 1: {}", part1(input.clone()));
    println!("part 2: {}", part2(input.clone()));
}

#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

#[derive(Debug)]
struct Reveal {
    red: u32,
    blue: u32,
    green: u32,
}

fn part1(input: String) -> u32 {
    let red_limit = 12;
    let blue_limit = 14;
    let green_limit = 13;

    input.lines().map(|line| parse_game(line.to_string()))
        .fold(0, |mut acc, game| {
            let is_possible = game.reveals.iter().fold(true, |acc, reveal| {
                acc && (reveal.red <= red_limit
                    && reveal.blue <= blue_limit
                    && reveal.green <= green_limit)
            });
            if is_possible {
                acc += game.id;
            }
            acc
        })
}

fn part2(input: String) -> u32 {
    input.lines().map(|line| parse_game(line.to_string()))
        .map(|game| {
            let minimums = game.reveals.iter().fold((0,0,0), |mut acc, reveal| {
                if reveal.red > acc.0 {
                    acc.0 = reveal.red;
                }
                if reveal.blue > acc.1 {
                    acc.1 = reveal.blue;
                }
                if reveal.green > acc.2 {
                    acc.2 = reveal.green;
                }
                acc
            });
            minimums.0 * minimums.1 * minimums.2
        }).sum()
}

fn parse_game(input: String) -> Game {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let split: Vec<&str> = input.split(":").collect();
    // Game 1
    let game_id = split[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();
    // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let game_reveals = split[1]
        .split(";")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            line.split(",").fold(Reveal {red: 0, blue: 0, green: 0}, |mut acc, v| {
                let cubes = v.trim().split(" ").collect::<Vec<&str>>();

                match cubes[1] {
                    "red" => acc.red = cubes[0].parse::<u32>().unwrap(),
                    "blue" => acc.blue = cubes[0].parse::<u32>().unwrap(),
                    "green" => acc.green = cubes[0].parse::<u32>().unwrap(),
                    _ => (),
                };

                acc
            })
        })
        .collect::<Vec<Reveal>>();

    Game {
        id: game_id,
        reveals: game_reveals,
    }
}
