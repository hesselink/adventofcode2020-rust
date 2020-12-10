use std::fs;

pub fn solve() {
    let input = fs::read_to_string("input/1").expect("Error reading file");
    let ls : Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    'outer: for i in 0..ls.len() {
        for j in i+1..ls.len() {
            if ls[i] + ls[j] == 2020 {
                println!("{}", ls[i] * ls[j]);
                break 'outer;
            }
        }
    }

    'outer2: for i in 0..ls.len() {
        for j in i+1..ls.len() {
            for k in j+1..ls.len() {
                if ls[i] + ls[j] + ls[k] == 2020 {
                    println!("{}", ls[i] * ls[j] * ls[k]);
                    break 'outer2;
                }
            }
        }
    }
}
