use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

pub fn solve() {
    let input = fs::read_to_string("input/2").expect("Error reading file");
    let result = input.lines().map(|s| parse(s)).filter(|i| is_valid(i)).count();
    println!("{}", result);

    let result2 = input.lines().map(|s| parse(s)).filter(|i| is_valid2(i)).count();
    println!("{}", result2);
}

#[derive(Debug)]
struct Input {
    min: usize,
    max: usize,
    chr: char,
    password: String,
}

fn parse(input: &str) -> Input {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    }
    let cap = RE.captures(input).unwrap();
    return Input {
        min: cap[1].parse().unwrap(),
        max: cap[2].parse().unwrap(),
        chr: cap[3].chars().next().unwrap(),
        password: cap[4].to_string(),
    }
}

fn is_valid(input: &Input) -> bool {
    let char_count = input.password.chars().filter(|&c| c == input.chr).count();
    return char_count >= input.min && char_count <= input.max;
}

fn is_valid2(input: &Input) -> bool {
    let chr1 = input.password.chars().nth(input.min - 1).unwrap();
    let chr2 = input.password.chars().nth(input.max - 1).unwrap();
    return (chr1 == input.chr) != (chr2 == input.chr);
}
