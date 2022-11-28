use std::{fs};

#[derive(PartialEq, Eq)]
#[derive(Clone)]
#[derive(Debug)]
enum Cucumber {
    None,
    Right,
    Bottom
}

fn parse(content: String) -> Vec<Vec<Cucumber>> {
    let mut res: Vec<Vec<Cucumber>> = vec!();
    let lines: Vec<&str> = content.lines().collect();
    for line in lines {
        let mut line_vec = vec!();
        for char in line.as_bytes() {
            match char {
                &b'>' => line_vec.push(Cucumber::Right),
                &b'v' => line_vec.push(Cucumber::Bottom),
                &b'.' => line_vec.push(Cucumber::None),
                _ => panic!("unrecognized character")
            }
        }
        res.push(line_vec);
    }
    res
}

fn move_cucumber(cucumber: Cucumber, map: &mut Vec<Vec<Cucumber>>, x: usize, y: usize, done: &mut Vec<Vec<bool>>, checkmap: &Vec<Vec<Cucumber>>) -> bool {
    let res = match cucumber {
        Cucumber::Right => {
            let next_x = if x >= map[y].len() - 1 { 0 } else { x + 1 };
            match checkmap[y][next_x] {
                Cucumber::None => {
                    map[y][next_x] = cucumber.clone();
                    done[y][next_x] = true;
                    map[y][x] = Cucumber::None;
                    true
                }
                _ => false
            }
        },
        Cucumber::Bottom => {
            let next_y = if y >= map.len() - 1 { 0 } else { y + 1 };
            match checkmap[next_y][x] {
                Cucumber::None => {
                    map[next_y][x] = cucumber.clone();
                    done[next_y][x] = true;
                    map[y][x] = Cucumber::None;
                    true
                }
                _ => false
            }
        },
        _ => false
    };
    res
}

fn cont(map: &mut Vec<Vec<Cucumber>>) -> bool {
    let check_map = map.clone();
    let mut res = 0;
    let mut done = vec![vec![false; map[0].len()]; map.len()];
    for y in 0..map.len() {
        // dbg!(map.len());
        for x in 0..map[y].len() {
            if done[y][x] == true {
                continue;
            }
            let cucumber = map[y][x].clone();
            if cucumber != Cucumber::Right {
                continue;
            }
            if move_cucumber(cucumber, map, x, y, &mut done, &check_map) == true {
                res = res + 1
            }
        }
    }

    let mut done = vec![vec![false; map[0].len()]; map.len()];
    let check_map = map.clone();

    for y in 0..map.len() {
        // dbg!(map.len());
        for x in 0..map[y].len() {
            // dbg!(x);
            // dbg!(y);
            if done[y][x] == true {
                continue;
            }
            let cucumber = map[y][x].clone();
            if cucumber != Cucumber::Bottom {
                continue;
            }
            if move_cucumber(cucumber, map, x, y, &mut done, &check_map) == true {
                res = res + 1
            }
        }
    }

    res != 0
}

fn display(map: &Vec<Vec<Cucumber>>) {
    for y in 0..map.len() {
        // dbg!(map.len());
        for x in 0..map[y].len() {
            match map[y][x] {
                Cucumber::Right => print!(">"),
                Cucumber::Bottom => print!("v"),
                _ => print!(".")
            }
        }
        println!();
    }
    println!();
}

pub fn run() {
    let input_result = fs::read_to_string("src/input25.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut map = parse(input);
    let mut i: usize = 0;
    loop {
        // display(&map);
        let res = cont(&mut map);
        // std::io::stdin().read_line(&mut String::new());
        i = i + 1;
        if res == false {
            break;
        }
        // dbg!(i + 1);
    }
    dbg!(i);
}
