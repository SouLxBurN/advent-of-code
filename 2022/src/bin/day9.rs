use std::cell::RefCell;
use std::collections::HashSet;

type Knot = (i32, i32);

#[derive(Debug)]
enum Action {
    UP(usize),
    DOWN(usize),
    LEFT(usize),
    RIGHT(usize),
    NONE(usize),
}

impl Action {
    fn parse_action_line(line: &str) -> Action {
        let mut a_line_split = line.trim().split(" ");
        match a_line_split.next() {
            Some("U") => Action::UP(a_line_split.next().unwrap().parse::<usize>().unwrap()),
            Some("D") => Action::DOWN(a_line_split.next().unwrap().parse::<usize>().unwrap()),
            Some("L") => Action::LEFT(a_line_split.next().unwrap().parse::<usize>().unwrap()),
            Some("R") => Action::RIGHT(a_line_split.next().unwrap().parse::<usize>().unwrap()),
            _ => Action::NONE(0),
        }
    }
}

fn calculate_move(diff: (i32,i32)) -> (i32,i32) {
    match diff {
        ( 1,  2) | ( 2,  1) | ( 2,  2) => ( 1,  1),
        ( 2, -1) | ( 1, -2) | ( 2, -2) => ( 1, -1),
        (-2,  1) | (-1,  2) | (-2,  2) => (-1,  1),
        (-2, -1) | (-1, -2) | (-2, -2) => (-1, -1),
        ( 2,  0) => ( 1,  0),
        (-2,  0) => (-1,  0),
        ( 0,  2) => ( 0,  1),
        ( 0, -2) => ( 0, -1),
        _ => (0,0),
    }
}

fn move_tail(head: Knot, tail: &mut Knot) {
    let diff = (head.0 - tail.0, head.1 - tail.1);
    // println!("H:{:?},T:{:?} | D:{:?}", head, tail, diff);

    let mv = calculate_move(diff);
    tail.0 = tail.0 + mv.0;
    tail.1 = tail.1 + mv.1;
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day9.input").unwrap();
    let actions: Vec<Action> = input.trim().split("\n").map(|line| Action::parse_action_line(line)).collect();

    part1(&actions);
    part2(&actions);
}

fn part1(actions: &Vec<Action>) {
    let mut head: Knot = (0,0);
    let mut tail: Knot = (0,0);
    let mut visited: HashSet<(i32,i32)> = HashSet::new();

    for action in actions.iter() {
        match action {
            Action::UP(cnt) => {
                (0..*cnt).for_each(|_mv| {
                    head.0 += 1;
                    move_tail(head, &mut tail);
                    visited.insert(tail);
                });
            },
            Action::DOWN(cnt) => {
                (0..*cnt).for_each(|_mv| {
                    head.0 -= 1;
                    move_tail(head, &mut tail);
                    visited.insert(tail);
                });
            },
            Action::LEFT(cnt) => {
                (0..*cnt).for_each(|_mv| {
                    head.1 -= 1;
                    move_tail(head, &mut tail);
                    visited.insert(tail);
                });
            },
            Action::RIGHT(cnt) => {
                (0..*cnt).for_each(|_mv| {
                    head.1 += 1;
                    move_tail(head, &mut tail);
                    visited.insert(tail);
                });
            },
            _ => {},
        }
    }

    println!("Part 1: {}", visited.len());

}

fn part2(actions: &Vec<Action>) {
    let mut rope: Vec<RefCell<Knot>> = (0..=9).map(|_| {
        RefCell::new((0,0))
    }).collect();

    let mut visited: HashSet<(i32,i32)> = HashSet::new();

    for action in actions.iter() {
        match action {
            Action::UP(cnt) => {
                (0..*cnt).for_each(|_mv| {
                    rope.first_mut().unwrap().borrow_mut().0 += 1;
                    move_rope(&rope, &mut visited);
                });
            },
            Action::DOWN(cnt) => {
                (0..*cnt).for_each(|_mv| {
                    rope.first_mut().unwrap().borrow_mut().0 -= 1;
                    move_rope(&rope, &mut visited);
                });
            },
            Action::LEFT(cnt) => {
                (0..*cnt).for_each(|_mv| {
                    rope.first_mut().unwrap().borrow_mut().1 -= 1;
                    move_rope(&rope, &mut visited);
                })
            },
            Action::RIGHT(cnt) => {
                (0..*cnt).for_each(|_mv| {
                    rope.first_mut().unwrap().borrow_mut().1 += 1;
                    move_rope(&rope, &mut visited);
                });
            },
            _ => {},
        }
    }

    println!("Part 2: {}", visited.len());

}

fn move_rope(rope: &Vec<RefCell<Knot>>, visited: &mut HashSet<Knot>) {
    rope.windows(2).for_each(|knot_window| {
        move_tail(*knot_window[0].borrow(), &mut knot_window[1].borrow_mut());
    });
    visited.insert(*rope.last().unwrap().borrow());
}
