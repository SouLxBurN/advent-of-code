use itertools::{Itertools, FoldWhile};

//  ----- x
// | 30373
// | 25512
// | 65332
// | 33549
// | 35390
// y
fn main() {
    let input = std::fs::read_to_string("./src/bin/day8.input").unwrap();
    let trees = input.trim().split("\n").map(|row| {
        row.trim().split("").flat_map(|tree| {
            tree.parse::<u32>()
        }).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    part1(&trees);
    part2(&trees);
}

fn part1(trees: &Vec<Vec<u32>>) {
    let count = trees.iter().enumerate().fold(0, |mut visible, (y, row)| {
        visible += row.iter().enumerate().fold(0, |visible, (x, tree_ht)| {
            if y == 0 || y == trees.len()-1 { return visible+1; }
            if x == 0 || x == row.len()-1 { return visible+1; }

            // println!("({},{}) H: {}", y, x, tree_ht);

            let vis = (0..x).rev().fold(true, |vis, x_in| {
                if !vis {
                    return vis;
                } else if trees[y][x_in] >= *tree_ht {
                    return false;
                }
                vis
            });

            let vis = vis || (x+1..row.len()).fold(true, |vis, x_in| {
                if !vis {
                    return vis;
                } else if trees[y][x_in] >= *tree_ht {
                    return false;
                }
                vis
            });

            let vis = vis || (0..y).rev().fold(true, |vis, y_in| {
                if !vis {
                    return vis;
                } else if trees[y_in][x] >= *tree_ht {
                    return false;
                }
                vis
            });

            let vis = vis || (y+1..trees.len()).fold(true, |vis, y_in| {
                if !vis {
                    return vis;
                } else if trees[y_in][x] >= *tree_ht {
                    return false;
                }
                vis
            });

            if vis {
                visible+1
            } else {
                visible
            }
        });
        visible
    });

    println!("Part 1: {:?}", count);
}

fn part2(trees: &Vec<Vec<u32>>) {
    let high_score = trees.iter().enumerate().fold(0, |score, (y, row)| {
        let tmp_score = row.iter().enumerate().fold(0, |score_acc, (x, tree_ht)| {
            // println!("({},{}) H: {}", y, x, tree_ht);

            let score_left = (0..x).rev().fold_while(0, |t_score, x_in| {
                if trees[y][x_in] < *tree_ht {
                    return FoldWhile::Continue(t_score+1);
                }
                FoldWhile::Done(t_score+1)
            });

            let score_right = (x+1..row.len()).fold_while(0, |t_score, x_in| {
                if trees[y][x_in] < *tree_ht {
                    return FoldWhile::Continue(t_score+1);
                }
                FoldWhile::Done(t_score+1)
            });

            let score_down = (0..y).rev().fold_while(0, |t_score, y_in| {
                if trees[y_in][x] < *tree_ht {
                    return FoldWhile::Continue(t_score+1);
                }
                FoldWhile::Done(t_score+1)
            });

            let score_up = (y+1..trees.len()).fold_while(0, |t_score , y_in| {
                if trees[y_in][x] < *tree_ht {
                    return FoldWhile::Continue(t_score+1);
                }
                FoldWhile::Done(t_score+1)
            });

            let total_score = score_left.into_inner()
                * score_right.into_inner()
                * score_up.into_inner()
                * score_down.into_inner();

            if score_acc < total_score {
                total_score
            } else {
                score_acc
            }
        });

        if score < tmp_score {
            tmp_score
        } else {
            score
        }
    });

    println!("Part 2: {:?}", high_score);
}
