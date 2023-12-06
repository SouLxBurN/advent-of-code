use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    dirs: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
    size: usize
}

impl Directory {
    fn new(name: String, parent: Option<Rc<RefCell<Directory>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Directory{
            name,
            parent,
            dirs: vec!(),
            files: vec!(),
            size: 0,
        }))
    }
}

#[derive(Debug)]
struct File {
    _name: String,
    _size: usize
}

#[derive(Debug)]
enum Command {
    CD(String),
    DIR(String),
    FILE(usize, String),
    LS,
    NULL,
}

impl Command {
    fn parse(line: &str) -> Self {
        let mut sp = line.split(" ");
        let c = sp.next().unwrap();
        match c {
            "$" => {
                match sp.next() {
                    Some("cd") => Command::CD(sp.next().unwrap().to_string()),
                    Some("ls") => Command::LS,
                    _ => panic!("Bad Input"),
                }
            },
            "dir" => {
                Command::DIR(sp.next().unwrap().to_string())
            },
            "" => {
                Command::NULL
            },
            _ => {
                Command::FILE(c.parse::<usize>().unwrap(), sp.next().unwrap().to_string())
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day7.input").unwrap();
    let root = Directory::new(String::from("/"), None);

    let _file_system = input.split("\n")
        .fold(root.clone(), |mut fs, line| {
            let cmd = Command::parse(line);
            // println!("{:?}", cmd);
            match cmd {
                Command::CD(name) => {
                    fs = match name.as_str() {
                        "/" => root.clone(),
                        ".." => fs.borrow().parent.as_ref().unwrap().clone(), // TODO: might not have a parent
                        _ => fs.borrow().dirs.iter().find(|dir| name == dir.borrow().name).unwrap().clone(),
                    }
                },
                Command::DIR(name) => fs.borrow_mut().dirs.push(Directory::new(name, Some(fs.clone()))),
                Command::FILE(size, name) => {
                    fs.borrow_mut().files.push(File{_size: size,_name: name});
                    let mut dir = fs.clone();
                    loop {
                        dir.borrow_mut().size += size;
                        if dir.borrow().parent.is_some() {
                            dir = dir.clone().borrow().parent.as_ref().unwrap().clone();
                        } else {
                            break;
                        }
                    }
                },
                Command::LS|Command::NULL => {},
            }
            fs
        });

    let space_needed = 30000000 - (70000000 - root.borrow().size);
    println!("Part 1: {}", part1(root.clone()));
    println!("Part 2({}): {}", space_needed, part2(root.clone(), space_needed));
}


fn part1(dir: Rc<RefCell<Directory>>) -> usize {
    // println!("{}: {}", dir.borrow().name, dir.borrow().size);
    let s = dir.borrow().size;
    if dir.borrow().dirs.len() == 0 {
        if s < 100000 {
            return s;
        } else {
            return 0;
        }
    } else {
        let mut child_sum = if s < 100000 {
            s
        } else {
            0
        };

        for cd in dir.borrow().dirs.iter() {
            child_sum += part1(cd.clone());
        }
        child_sum
    }
}

fn part2(dir: Rc<RefCell<Directory>>, space_needed: usize) -> usize {
    // println!("{}: {}", dir.borrow().name, dir.borrow().size);
    let s = dir.borrow().size;
    if dir.borrow().dirs.len() == 0 {
        return s;
    } else {
        let mut perfect_size = s;
        for cd in dir.borrow().dirs.iter() {
            let child_size = part2(cd.clone(), space_needed);
            if child_size < perfect_size && child_size > space_needed {
                perfect_size = child_size;
            }
        }
        perfect_size
    }
}
