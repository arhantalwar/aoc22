use std::{fs, collections::HashMap};

fn main() {

    let mut abc: HashMap<char, i32> = HashMap::new();

    for i in 97..123 as i32 {
        abc.insert(char::from(i as u8), i - 96);
    }

    for i in 65..91 as i32 {
        abc.insert(char::from(i as u8), i - 38);
    }

    let raw_file = fs::read_to_string("/home/arhant/aoc/rust/aoc3/src/file.txt").unwrap();
    let line_file: Vec<_> = raw_file.lines().map(|x| x.chars().collect::<Vec<_>>()).collect();

    let mut hashy: HashMap<char, i32> = HashMap::new();
    let mut same_vec: Vec<char> = Vec::new();

    line_file.iter().for_each(|x| {

        let mut is_found = false;
        hashy.clear();

        x.iter().enumerate().for_each(|(i, y)| {

            if i < x.len()/2 {
                hashy.entry(*y).or_insert(1);
            } else {
                if hashy.contains_key(y) && !is_found {
                    same_vec.push(*y);
                    is_found = true;
                }
            }

        });

    });

    let mut p_sum = 0;

    same_vec.iter().for_each(|x| {
        p_sum += abc.get(x).unwrap();
    });

    println!("Part 1 -> {}", p_sum);

    let line_file2: Vec<_> = raw_file.lines().map(|x| x.chars().collect::<Vec<_>>()).collect();
    let mut hashy3: HashMap<char, i32> = HashMap::new();
    let mut group3: Vec<char> = Vec::new();

    let three_group = line_file2.chunks(3);

    for group in three_group {

        let mut is_found = false;
        hashy3.clear();

        for (chunk_num, val) in group.iter().enumerate() {

            if chunk_num == 2 && !is_found {
                val.iter().for_each(|x| { hashy3.entry(*x).and_modify(|y| {
                    if *y == 2 {
                        *y += 1;
                        is_found = true;
                        group3.push(*x);
                    }
                });
                });
            } else if chunk_num == 1 {
                val.iter().for_each(|x| { hashy3.entry(*x).and_modify(|y| {
                    if *y == 1 {
                        *y += 1
                    }
                });
                });
            } else {
                val.iter().for_each(|x| { hashy3.entry(*x).or_insert(1); });
            }

        }

    }

    p_sum = 0;

    group3.iter().for_each(|x| {
        p_sum += abc.get(x).unwrap();
    });

    println!("Part 2 -> {:?}", p_sum);

}
