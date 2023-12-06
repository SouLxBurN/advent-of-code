#[derive(Debug)]
struct Action {
    count: usize,
    from_col: usize,
    to_col: usize,
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day5.input").unwrap();
    let (i_stack, i_moves): (&str, &str) = input.split_once("\n\n").unwrap();
    let (stack_lines, col_nums) = i_stack.rsplit_once("\n").unwrap();

    let mut stacks: Vec<Vec<&str>> = col_nums
            .trim().split("   ")
            .map(|_|Vec::new())
            .collect();

    // Golf:
    // line.chars().chunks(4).into_iterator().enumerate().map(|(i, chunk) | stacks[col].extend(chunk.nth(1)).iter().filter(|c| c != ' '))
    stack_lines.rsplit("\n").for_each(|line| {
        let base_offset = 1;
        let offset = 4;

        (0..stacks.len()).for_each(|col| {
            let krate_offset = base_offset + (col*offset);
            let krate_id = &line[krate_offset..=krate_offset];
            if krate_id != " " {
                stacks[col].push(krate_id);
            }
        });
    });
    dbg!(&stacks);

    let actions: Vec<Action> = i_moves.trim().split("\n")
        .map(|action| {
            let mut ac_iter = action.split(" ")
                .filter_map(|w| w.parse::<usize>().ok());
            Action{
                count: ac_iter.next().unwrap(),
                from_col: ac_iter.next().unwrap(),
                to_col: ac_iter.next().unwrap()
            }
        }).collect();

    dbg!(&actions);

    part1(stacks.clone(), &actions);
    part2(stacks.clone(), &actions);
}

fn part1(mut stacks: Vec<Vec<&str>>, actions: &Vec<Action>) {
    for action in actions {
       (0..action.count).for_each(|_| {
            let krate = stacks[action.from_col-1].pop().unwrap();
            stacks[action.to_col-1].push(krate);
        });
    }

    let top_crates = stacks.iter().fold(String::from(""), |mut acc, stack| {
        acc.push_str(&stack.last().unwrap());
        acc
    });

    println!("Part 1: {}", top_crates);
}

fn part2(mut stacks: Vec<Vec<&str>>, actions: &Vec<Action>) {
    for action in actions {
        let stack = &mut stacks[action.from_col-1];
        let mut krate: Vec<&str> = stack.drain(stack.len()-action.count..).collect();
        stacks[action.to_col-1].append(&mut krate);
    }

    let top_crates = stacks.iter().fold(String::from(""), |mut acc, stack| {
        acc.push_str(&stack.last().unwrap());
        acc
    });

    println!("Part 2: {}", top_crates);
}
