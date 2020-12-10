use std::fs;

pub fn solve() {
    let input = fs::read_to_string("input/3").expect("Error reading file");
    let map : Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    println!("{}", count_trees(&map, 3, 1));

    let result2 : i64 = [(1usize,1usize), (3,1), (5,1), (7,1), (1,2)].iter().map(|(x, y)| count_trees(&map, *x, *y)).product();
    println!("{}", result2);
}

fn count_trees(map: &Vec<Vec<char>>, right: usize, down: usize) -> i64 {
    let width = map[0].len();
    let mut tree_count = 0;
    let mut x;
    for y in (0..map.len()).step_by(down) {
        x = y * right / down % width;
        if map[y][x] == '#' {
            tree_count += 1;
        }
    }
    return tree_count;
}
