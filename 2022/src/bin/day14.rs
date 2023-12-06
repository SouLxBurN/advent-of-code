const AIR: char = '.';
const ROCK: char = '#';
const SAND: char = 'O';

type Line = Vec<Pt>;

#[derive(Debug, Clone)]
struct Pt {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Dimensions {
    min_x: usize,
    max_x: usize,
    max_y: usize,
}
impl Dimensions {
    fn new() -> Self {
        Dimensions {
            min_x: 1000,
            max_x: 0,
            max_y: 0,
        }
    }

    fn x(&self, og_x: usize) -> usize {
        og_x - (self.min_x)
    }

    fn is_oob(&self, y: usize, x: usize) -> bool {
        if y >= self.max_y {
            return true;
        } else if x > self.max_x || x < self.min_x {
            return true;
        }
        false
    }
}

struct Cave {
    dims: Dimensions,
    grid: Vec<Vec<char>>,
}
impl Cave {
    /// Part 1 Cave
    fn new(dims: Dimensions) -> Self {
        let grid = vec![vec![AIR; dims.max_x - dims.min_x]; dims.max_y];
        Cave { dims, grid }
    }

    /// Part 2 Cave
    fn new_rock_bottom(dims: Dimensions) -> Self {
        let mut cave = Self::new(dims);
        cave.grid[cave.dims.max_y-1] = vec![ROCK; cave.dims.max_x - cave.dims.min_x];
        cave
    }

    fn add_sand(&mut self) -> bool {
        let mut sand_pt = Pt{x: 500, y:0};
        if self.grid[sand_pt.y][self.dims.x(sand_pt.x)] != AIR {
            return false;
        }

        self.grid[sand_pt.y][self.dims.x(sand_pt.x)] = SAND;
        loop {
            // Straight Down.
            if self.dims.is_oob(sand_pt.y+1, sand_pt.x) {
                break false;
            }
            if self.grid[sand_pt.y+1][self.dims.x(sand_pt.x)] == AIR {
                self.grid[sand_pt.y][self.dims.x(sand_pt.x)] = AIR;
                self.grid[sand_pt.y+1][self.dims.x(sand_pt.x)] = SAND;
                sand_pt.y += 1;
                continue;
            }

            // Down to the Left.
            if self.dims.is_oob(sand_pt.y+1, sand_pt.x-1) {
                break false;
            }
            if self.grid[sand_pt.y+1][self.dims.x(sand_pt.x-1)] == AIR {
                self.grid[sand_pt.y][self.dims.x(sand_pt.x)] = AIR;
                self.grid[sand_pt.y+1][self.dims.x(sand_pt.x-1)] = SAND;
                sand_pt.y += 1;
                sand_pt.x -= 1;
                continue;
            }

            // Down to the Right.
            if self.dims.is_oob(sand_pt.y+1, sand_pt.x+1) {
                break false;
            }
            if self.grid[sand_pt.y+1][self.dims.x(sand_pt.x+1)] == AIR {
                self.grid[sand_pt.y][self.dims.x(sand_pt.x)] = AIR;
                self.grid[sand_pt.y+1][self.dims.x(sand_pt.x+1)] = SAND;
                sand_pt.y += 1;
                sand_pt.x += 1;
                continue;
            }
            // No moves left, sand successfully settled.
            break true;
        }
    }

    fn add_lines(&mut self, lines: &Vec<Line>) {
        lines.iter().for_each(|line| {
            self.draw_line(&line);
        });
    }

    fn draw_line(&mut self, line: &Line) {
        let mut pt_iter = line.iter();
        let mut start = (*pt_iter.next().unwrap()).clone();

        self.grid[start.y][self.dims.x(start.x)] = ROCK;
        while let Some(end) = pt_iter.next() {
            if start.x > end.x {
                while start.x > end.x && start.y == end.y {
                    self.grid[start.y][self.dims.x(start.x)] = ROCK;
                    start.x -= 1;
                }
            } else if start.x < end.x && start.y == end.y {
                while start.x < end.x {
                    self.grid[start.y][self.dims.x(start.x)] = ROCK;
                    start.x += 1;
                }
            } else if start.y > end.y && start.x == end.x {
                while start.y > end.y {
                    self.grid[start.y][self.dims.x(start.x)] = ROCK;
                    start.y -= 1;
                }
            } else if start.y < end.y && start.x == end.x {
                while start.y < end.y {
                    self.grid[start.y][self.dims.x(start.x)] = ROCK;
                    start.y += 1;
                }
            } else {
                panic!("Diagonal Movement!");
            }
            start = end.clone();
        }
        self.grid[start.y][self.dims.x(start.x)] = ROCK;
    }

    fn draw(&self) {
        print!("{esc}[2J", esc="\x1b");
        self.grid.iter().for_each(|layer| {
            layer.iter().for_each(|c| {
                print!("{}", c);
            });
            print!("\n");
        });
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day14.input").unwrap();
    let lines: Vec<Line> = input
        .trim()
        .split("\n")
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let mut p = point.split(",");
                    Pt {
                        x: p.next().unwrap().parse::<usize>().unwrap(),
                        y: p.next().unwrap().parse::<usize>().unwrap(),
                    }
                })
                .collect::<Line>()
        })
        .collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

}

fn part2(lines: &Vec<Line>) -> i32 {
    let mut dimensions = lines.iter().fold(Dimensions::new(), |mut dims, line| {
        for pt in line.iter() {
            if pt.y >= dims.max_y {
                dims.max_y = pt.y + 3;
            }
        }
        dims
    });

    dimensions.max_x = (500f32 + (dimensions.max_y as f32 * 3f32.sqrt())).ceil() as usize;
    dimensions.min_x = (500f32 - (dimensions.max_y as f32 * 3f32.sqrt())).ceil() as usize;

    let mut cave = Cave::new_rock_bottom(dimensions);
    cave.add_lines(&lines);
    let mut sand_cnt = 0;
    while cave.add_sand() {
        sand_cnt += 1;
    }
    sand_cnt

}

fn part1(lines: &Vec<Line>) -> i32 {
    let dimensions = lines.iter().fold(Dimensions::new(), |mut dims, line| {
        for pt in line.iter() {
            if pt.x <= dims.min_x {
                dims.min_x = pt.x;
            }
            if pt.x >= dims.max_x {
                dims.max_x = pt.x + 1;
            }
            if pt.y >= dims.max_y {
                dims.max_y = pt.y + 1;
            }
        }
        dims
    });

    println!("{:?}", dimensions);
    let mut cave = Cave::new(dimensions);
    cave.add_lines(&lines);
    let mut sand_cnt = 0;
    while cave.add_sand() {
        sand_cnt += 1;
    }
    cave.draw();
    sand_cnt
}
