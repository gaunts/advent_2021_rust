use std::collections::HashSet;
use std::collections::HashMap;

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

fn backward(xcheck: i32, yadd: i32, zdiv: i32, z_final: i32, w: i32) -> Vec<i32> {
    let mut zs = vec!();
    let x = z_final - w - yadd;

    if x % 26 == 0 {
        zs.push(x / 26 * zdiv);
    }
    if 0 <= (w - xcheck) && (w - xcheck) < 26 {
        let z0 = z_final * zdiv;
        zs.push(w - xcheck + z0);
    }

    zs
}

fn solve(part: u8, all_div_check_add: Vec<(i32, i32, i32)>) {
    let mut zs = HashSet::new();
    zs.insert(0);
    let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut ws: Vec<i32> = (1..10).collect();
    if part == 2 {
        ws.reverse();
    }

    for (zdiv, xcheck, yadd) in all_div_check_add {
        let mut newzs: HashSet<i32> = HashSet::new();
        for w in &ws {
            for z in &zs {
                let z0s = backward(xcheck, yadd, zdiv, *z, *w);
                for z0 in z0s {
                    newzs.insert(z0);
                    let mut vec = vec!(*w);
                    if let Some(other) = result.get(z) {
                        vec.extend(other);
                    }
                    result.insert(z0, vec);
                }
            }
        }
        zs = newzs;
    }
    dbg!(result.get(&0).unwrap());
}

pub fn run() {
    std::env::set_var("RUST_BACKTRACE", "1");

    // if let Some(content) = fs::read_to_string("C:/Users/simon/source/repos/Rust/advent_2021/src/input.txt").ok() {
    let greeting_file_result = std::fs::read_to_string("src/input24.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut args = parse(greeting_file);
    args.reverse();

    solve(2, args);
}