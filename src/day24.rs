use std::fs;

fn op(mut z:i32, w:i32, args:(i32, i32, i32)) -> i32 {

    // if (z % 26) + args.1 != w {
    //     z = z / args.0;
    //     z = z * 26;
    //     z = z + (w + args.2)
    // }
    // else {
    //     z = z / args.0
    // }
    // z

    let x = z % 26;
    z /= args.0;
    if x != (w - args.1) {
        z = 26 * z + w + args.2
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
    weight: i32
}

fn cont(paths: Vec<Path>, args:(i32, i32, i32), mult: i32) -> Vec<Path>{
    dbg!(args);
    let mut new_paths: Vec<Path> = vec!();
    for path in paths {
        let last_val = path.cases[path.cases.len() - 1];
        for w in 1..10 {
            for zpos in 0..27 { // On est bien d'accords que z est toujours entre 0 et 26?
                if op(zpos, w, args) == last_val {
                    // dbg!(last_val);
                    let mut cases = path.cases.clone();
                    cases.push(zpos);
                    let new_path = Path {cases, weight: path.weight + (w * mult)};
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
    // if let Some(content) = fs::read_to_string("C:/Users/simon/source/repos/Rust/advent_2021/src/input.txt").ok() {
    let greeting_file_result = fs::read_to_string("src/input.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut args = parse(greeting_file);
    args.reverse();
    let mut paths = vec!(Path {cases : vec!(0), weight : 0});
    for mult in 0..4 { // Ca marche jusqu'à 3 mais pas 4
        paths = cont(paths, args[mult], 10_i32.pow(mult as u32));
    }
    println!("{}", paths.len());
}
