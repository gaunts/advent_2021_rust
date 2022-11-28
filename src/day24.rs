use std::fs;

// inp w
// mul x 0
// add x z
// mod x 26
// div z 26
// add x -2
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 15
// mul y x
// add z y
fn op_big(mut z:i32, w:i32, args:(i32, i32, i32)) -> i32 {
    let mut x = z;
    x = x % 26;
    z = z / args.0;
    x = x + args.1;
    if x == w {
        x = 1;
    }
    else {
        x = 0;
    }
    if x == 0 {
        x = 1;
    }
    else {
        x = 0;
    }
    let mut y = 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = 0;
    y = y + w;
    y = y + args.2;
    y = y * x;
    z = z + y;
    z
}

fn op(mut z:i32, w:i32, args:(i32, i32, i32)) -> i32 {
    let (zdiv, xcheck, yadd) = args;
    let x = z % 26;
    z /= zdiv;
    if x != (w - xcheck) {
        z = 26 * z + w + yadd
    }
    z
}

fn parse(content: String) -> Vec<(i32, i32, i32)> {
    let mut res: Vec<(i32, i32, i32)> = vec!();
    let lines: Vec<&str> = content.lines().collect();
    for i in 0..14 {
        if let (Some(a), Some(b), Some(c)) = 
            (lines[(18 * i) + 4].split(' ').collect::<Vec<&str>>()[2].parse::<i32>().ok(),
             lines[(18 * i) + 5].split(' ').collect::<Vec<&str>>()[2].parse::<i32>().ok(),
             lines[(18 * i) + 15].split(' ').collect::<Vec<&str>>()[2].parse::<i32>().ok()) {
            res.push((a, b, c));
        }
    }
    res
}

#[derive(Debug)]
struct Path {
    cases: Vec<i32>,
    weight: u64
}

fn cont(paths: Vec<Path>, args:(i32, i32, i32), mult: u64) -> Vec<Path>{
    dbg!(args);
    let mut new_paths: Vec<Path> = vec!();
    for path in paths {
        let last_val = path.cases[path.cases.len() - 1];
        for w in 1..10 {
            for zpos in -100..100 { // On est bien d'accords que z est toujours entre 0 et 25?
                if op(zpos, w, args) == last_val {
                    // dbg!(last_val);
                    let mut cases = path.cases.clone();
                    cases.push(zpos);
                    let new_path = Path {cases, weight: path.weight + (w as u64 * mult)};
                    // dbg!(&new_path);
                    new_paths.push(new_path);
                }
            }
        }
    }
    // dbg!(&new_paths);
    new_paths
}

pub fn run() {
    std::env::set_var("RUST_BACKTRACE", "1");

    // if let Some(content) = fs::read_to_string("C:/Users/simon/source/repos/Rust/advent_2021/src/input.txt").ok() {
    let greeting_file_result = fs::read_to_string("src/input24.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut args = parse(greeting_file);
    args.reverse();
    let mut paths = vec!(Path {cases : vec!(0), weight : 0});
    for mult in 0..14 { // Ca marche jusqu'à 3 mais pas 4
        let pow = 10_u64.pow(mult as u32);
        paths = cont(paths, args[mult], pow);
        println!("{}", paths.len());
    }
}
