use std::collections::HashSet;
use itertools::{Itertools, FoldWhile};

fn main() {
    let input = std::fs::read_to_string("./src/bin/day6.input").unwrap();

    println!("Part 1: {}", scan(&input, 4));
    println!("Part 2: {}", scan(&input, 14));
}

fn og_part1(input: &str) {
    let start = input.char_indices()
        .tuple_windows::<((usize,char), (usize,char), (usize,char), (usize,char))>()
        .fold_while(0usize, |acc, tuple| {
            let mut check_dupes: HashSet<char> = HashSet::new();
            check_dupes.insert(tuple.0.1);

            let mut exists = check_dupes.insert(tuple.1.1);
            if !exists {
                return FoldWhile::Continue(acc);
            }

            exists = check_dupes.insert(tuple.2.1);
            if !exists {
                return FoldWhile::Continue(acc);
            }

            exists = check_dupes.insert(tuple.3.1);
            if !exists {
                return FoldWhile::Continue(acc);
            }
            FoldWhile::Done(tuple.3.0 + 1)
        });
    println!("Part 1: {}", start.into_inner());
}

fn scan(input: &str, window: usize) -> usize {
    let chars: Vec<(usize,char)> = input.char_indices().collect();

    let mut seen: HashSet<char> = HashSet::new();
    for window in chars.windows(window) {
        if window.iter().fold(true, |all_unique, c| all_unique && seen.insert(c.1)) {
            return window.last().unwrap().0 + 1;
        }
        seen.clear();
    };
    0usize
}
