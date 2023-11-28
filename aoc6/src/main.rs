use std::{fs, collections::HashMap};

fn main() {

    let raw_file = fs::read_to_string("/home/arhant/aoc/rust/aoc6/src/file.txt").unwrap();
    let signal: Vec<char> = String::from(raw_file.trim()).chars().collect();

    let itr_p1 = signal.windows(4);
    let itr_p2 = signal.windows(14);

    let mut hashy: HashMap<char, i32> = HashMap::new();
    let mut char_num = 0;

    for i in itr_p1 {
        hashy.clear();
        i.iter().for_each(|x| { hashy.entry(*x).and_modify(|y| *y += 1).or_insert(1); });
        char_num += 1;
        if hashy.len() == 4 {
            char_num += 3;
            println!("Part 1 -> {char_num}");
            break;
        }
    }

    char_num = 0;

    for i in itr_p2 {
        hashy.clear();
        i.iter().for_each(|x| { hashy.entry(*x).and_modify(|y| *y += 1).or_insert(1); });
        char_num += 1;
        if hashy.len() == 14 {
            char_num += 13;
            println!("Part 2 -> {char_num}");
            break;
        }
    }

}
