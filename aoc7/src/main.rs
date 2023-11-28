use std::{fs, collections::HashMap};

fn main() {


    let raw_file = fs::read_to_string("/home/arhant/aoc/rust/aoc7/src/file.txt").unwrap();

    let mut stack: Vec<String> = Vec::new();
    let mut hashy: HashMap<String, i64> = HashMap::new();

    for i in raw_file.lines() {

        let command = i.split_whitespace().collect::<Vec<&str>>();

        if command.len() == 3 {
            if command[2] == ".." {
                stack.pop();
            } else {
                let path = if let Some(top) = stack.last() {
                    format!("{}_{}", top, command[2])
                } else {
                    command[2].to_string()
                };
                stack.push(path.clone());
            }
        } else if command.len() == 2 {

            if command[0] != "$" && command[0] != "dir" {

                let size = command[0].parse::<i64>().unwrap();

                for i in &stack {
                    hashy.entry(i.clone()).and_modify(|x| *x += size).or_insert(size);
                }

            }

        }

    }

    let mut total_size: i64 = 0;
    let mut sizes: Vec<i64> = Vec::new();

    hashy.iter().for_each(|(_, y)| { if *y <= 100000 {
            total_size += y;
    } 
            sizes.push(*y);
    });

    println!("Part 1 -> {total_size}");

    let needed_size = 30000000 - (70000000 - hashy.get("/").unwrap());

    sizes.sort();

    for i in &sizes {
        if *i > needed_size {
            println!("Part 2 -> {i}");
            break;
        }
    }

}






