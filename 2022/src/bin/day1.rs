fn main() {
    let input = std::fs::read_to_string("./src/bin/day1.input").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let elf_w_most = input.split("\n\n").map(|elf_cals| {
        elf_cals.split("\n").flat_map(|cal| {
            cal.parse::<u32>()
        }).sum::<u32>()
    }).max().unwrap();
    println!("{}", elf_w_most);
}

fn part2(input: &str) {
    let mut elfs_w_most: Vec<u32> = input.split("\n\n").map(|elf_cals| {
        elf_cals.split("\n").flat_map(|cal| {
            cal.parse::<u32>()
        }).sum::<u32>()
    }).collect();

    elfs_w_most.sort_by(|a,b| b.cmp(a));
    let top_3: u32 = elfs_w_most.iter().take(3).sum();
    println!("{}", top_3);
}
