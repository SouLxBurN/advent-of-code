#[derive(Debug)]
enum Instruction {
    ADDX(i32),
    NOOP
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day10.input").unwrap();
    let instructions: Vec<Instruction> = input.trim().split("\n").map(|line| {
        let mut split = line.trim().split(" ");
        match split.next().unwrap() {
            "addx" => Instruction::ADDX(split.next().unwrap().parse::<i32>().unwrap()),
            _ => Instruction::NOOP
        }
    }).collect();

    part1(&instructions);
    part2(&instructions);
}

fn part1(instructions: &Vec<Instruction>) {
    let mut signal_strengths: Vec<(i32,i32)> = vec![]; // (20, X), (40, X2), (80, X3)...

    let mut cycle = 0i32;
    let mut reg_x = 1i32;
    let mut busy = 0;
    let mut instr = instructions.iter().peekable();

    // let mut execution = instr.next().unwrap();
    let mut execution = &Instruction::NOOP;
    while cycle <= 220 && instr.peek().is_some() {
        // Compute Signal Strengths
        match cycle {
            20 => signal_strengths.push((20, reg_x*cycle)),
            _ if (cycle+20) % 40 == 0 => signal_strengths.push((cycle, reg_x*cycle)),
            _ => {}
        }

        // If instruction is in progress
        if busy <= 0 {
            println!("{:?}", execution);
            // Complete Instruction
            match execution {
                Instruction::ADDX(v) => {
                    reg_x += v;
                },
                Instruction::NOOP => {},
            }
            // Load Next Instruction
            execution = instr.next().unwrap();
            match execution {
                Instruction::ADDX(_) => busy += 1,
                Instruction::NOOP => busy += 0,
            }
        } else {
            busy -= 1;
        }

        cycle += 1;
        println!("{cycle}");
    }
    println!("{:?}", signal_strengths);
    println!("Part 1: {:?}", signal_strengths.iter().fold(0, |sum, strength| sum + strength.1));
}

fn part2(instructions: &Vec<Instruction>) {

    let mut output = String::from("");
    let mut cycle = 0i32;
    let mut reg_x = 1i32;
    let mut busy = 0;
    let mut instr = instructions.iter().peekable();

    // let mut execution = instr.next().unwrap();
    let mut execution = &Instruction::NOOP;
    while cycle <= 240 && instr.peek().is_some() {

        // If instruction is in progress
        if busy <= 0 {
            // Complete Instruction
            match execution {
                Instruction::ADDX(v) => {
                    reg_x += v;
                },
                Instruction::NOOP => {},
            }
            // Load Next Instruction
            execution = instr.next().unwrap();
            match execution {
                Instruction::ADDX(_) => busy += 1,
                Instruction::NOOP => busy += 0,
            }
        } else {
            busy -= 1;
        }

        // draw pixels
        let col = cycle % 40;
        if cycle != 0 && col == 0 {
            output.push_str("\n");
        }
        if col >= reg_x-1 && col <= reg_x+1 {
            output.push_str("█");
        } else {
            output.push_str(" ");
        }

        cycle += 1;
        // println!("{cycle}");
    }
    println!("Part 2:");
    println!("{output}");

}
