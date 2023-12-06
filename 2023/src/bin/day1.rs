fn main() {
    let input = std::fs::read_to_string("./src/bin/day1.input").expect("Input file not found");
    println!("part 1: {}", part1(input.clone()));
    println!("part 2: {}", part2(input.clone()));
}

fn part2(input: String) -> u32 {
    let numbers = vec!("one", "two", "three", "four", "five", "six", "seven", "eight", "nine");

    input.lines().map(|line| {
        let mut left_digit = 0;
        let mut right_digit = 0;
        let mut buffer = String::new();

        line.chars().for_each(|c| {
            if !c.is_digit(10) {
                buffer.push(c);
                let word = numbers.iter().fold("", |mut acc, x| {
                    if buffer.contains(x) {
                        acc = x;
                    }
                    acc
                });

                if word != "" {
                    if left_digit == 0 {
                        left_digit = convert_word_to_digit(word)*10;
                    }
                    right_digit = convert_word_to_digit(word);
                    buffer.clear();
                }
            } else {
                if left_digit == 0 {
                    left_digit = c.to_digit(10).unwrap()*10;
                }
                right_digit = c.to_digit(10).unwrap();
                buffer.clear();
            }
        });
        left_digit+right_digit
    }).sum::<u32>()
}

fn convert_word_to_digit(word: &str) -> u32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0
    }
}


fn part1(input: String) -> u32 {
    input.lines().map(|line| {
        let mut left_digit = 'x';
        let mut right_digit = 'x';

        line.chars().for_each(|c| {
            if c.is_digit(10) {
                if left_digit == 'x' {
                    left_digit = c
                }
                right_digit = c
            }
        });
        left_digit.to_digit(10).unwrap()*10+right_digit.to_digit(10).unwrap()
    }).sum::<u32>()
}
