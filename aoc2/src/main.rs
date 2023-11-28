use std::{fs, collections::HashMap, char};

fn main() {

    let raw_file = fs::read_to_string("/home/arhant/aoc/rust/aoc2/src/file.txt").unwrap();
    let mut hashy: HashMap<(char, char), i32> = HashMap::new();

    let f: Vec<_> = raw_file.lines().map(|x| x.chars().collect::<Vec<_>>()).collect();

    for i in f {
        hashy.entry((*i.get(0).unwrap(), *i.get(2).unwrap())).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut total_score = 0;

    for i in &hashy {
        match i {

            (('A', 'X'), x) => {
                total_score += (1 + 3) * x;
            }

            (('A', 'Y'), x) => {
                total_score += (2 + 6) * x;
            }

            (('A', 'Z'), x) => {
                total_score += (3 + 0) * x;
            }

            (('B', 'X'), x) => {
                total_score += (1 + 0) * x;
            }

            (('B', 'Y'), x) => {
                total_score += (2 + 3) * x;
            }

            (('B', 'Z'), x) => {
                total_score += (3 + 6) * x;
            }

            (('C', 'X'), x) => {
                total_score += (1 + 6) * x;
            }

            (('C', 'Y'), x) => {
                total_score += (2 + 0) * x;
            }

            (('C', 'Z'), x) => {
                total_score += (3 + 3) * x;
            }

            _ => {}

        }

    }

    println!("Part 1 -> {}", total_score);

    // A, X -> Lose (1) Rock
    // B, Y -> draw (2) Paper
    // C, Z -> win (3) Scissor

    total_score = 0;

    for i in &hashy {
        match i {

            (('A', 'X'), x) => {
                total_score += (3 + 0) * x;
            }

            (('A', 'Y'), x) => {
                total_score += (1 + 3) * x;
            }

            (('A', 'Z'), x) => {
                total_score += (2 + 6) * x;
            }

            (('B', 'X'), x) => {
                total_score += (1 + 0) * x;
            }

            (('B', 'Y'), x) => {
                total_score += (2 + 3) * x;
            }

            (('B', 'Z'), x) => {
                total_score += (3 + 6) * x;
            }

            (('C', 'X'), x) => {
                total_score += (2 + 0) * x;
            }

            (('C', 'Y'), x) => {
                total_score += (3 + 3) * x;
            }

            (('C', 'Z'), x) => {
                total_score += (1 + 6) * x;
            }

            _ => {}

        }

    }

    println!("Part 2 -> {}", total_score);

}
