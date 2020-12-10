use std::fs;
use std::collections::HashMap;

pub fn solve() {
    let input = fs::read_to_string("input/4").expect("Error reading file");
    let ls = input.lines().collect::<Vec<&str>>();
    let result = ls
        .split(|l| l.is_empty())
        .map(|g| parse_group(g))
        .filter(|m| is_valid(m))
        .count();
    println!("{}", result);
}

fn parse_group(g: &[&str]) -> HashMap<String, String> {
    let mut m = HashMap::new();
    for l in g.iter() {
        l.split(" ").for_each(|kv| insert_kv(&mut m, kv));
    }
    return m;
}

fn insert_kv(m: &mut HashMap<String, String>, kv: &str) {
    let ix = kv.find(':').unwrap();
    m.insert(kv[..ix].to_string(), kv[ix..].to_string());
}

fn is_valid(m: &HashMap<String, String>) -> bool {
    return ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().all(|k| m.get(*k).is_some());
}
