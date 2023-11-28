use std::fs;

fn main() {

    let mut stack: Vec<Vec<char>> = Vec::new();

    let one: Vec<char> = vec!['Z', 'J', 'N', 'W', 'P', 'S'];
    let two: Vec<char> = vec!['G', 'S', 'T'];
    let three: Vec<char> = vec!['V', 'Q', 'R', 'L', 'H'];
    let four: Vec<char> = vec!['V', 'S', 'T', 'D'];
    let five: Vec<char> = vec!['Q', 'Z', 'T', 'D', 'B', 'M', 'J'];
    let six: Vec<char> = vec!['M', 'W', 'T', 'J', 'D', 'C', 'Z', 'L'];
    let seven: Vec<char> = vec!['L', 'P', 'M', 'W', 'G', 'T', 'J'];
    let eight: Vec<char> = vec!['N', 'G', 'M', 'T', 'B', 'F', 'Q', 'H'];
    let nine: Vec<char> = vec!['R', 'D', 'G', 'C', 'P', 'B', 'Q', 'W'];

    stack.push(one);
    stack.push(two);
    stack.push(three);
    stack.push(four);
    stack.push(five);
    stack.push(six);
    stack.push(seven);
    stack.push(eight);
    stack.push(nine);

    let mut commands_vec: Vec<i32> = Vec::new();

    let raw_file = fs::read_to_string("/home/arhant/aoc/rust/aoc5/src/file.txt").unwrap();

    let _ = raw_file.lines().for_each(|x| {

        let mut a: Vec<_> = x.split(" ").collect();

        a.iter().enumerate().for_each(|(i, val)| {
            if i == 1 || i == 3 || i == 5 {
                commands_vec.push(val.parse().unwrap());
            }
        });

        a.clear();

    });

    let procedure = commands_vec.chunks(3);

    for i in procedure {
        carter_9001(i, &mut stack)
    }

    let part_one = get_top_elem_from_stack(&stack);

    println!("Part 2 -> {:?}", part_one);

}

fn carter_9000(procedure: &[i32], stack: &mut Vec<Vec<char>>) {

    let a = procedure[0];
    let b = procedure[1] - 1;
    let c = procedure[2] - 1;

    for _ in 0..a {
        let pop_elem = stack.get_mut(b as usize).unwrap().pop().unwrap();
        stack.get_mut(c as usize).unwrap().push(pop_elem);
    }

}

fn carter_9001(procedure: &[i32], stack: &mut Vec<Vec<char>>) {

    let a = procedure[0];
    let b = procedure[1] - 1;
    let c = procedure[2] - 1;

    if a == 1 {

        for _ in 0..a {
            let pop_elem = stack.get_mut(b as usize).unwrap().pop().unwrap();
            stack.get_mut(c as usize).unwrap().push(pop_elem);
        }

    } else {

        let mut carter_vec: Vec<char> = Vec::new();

        for _ in 0..a {
            let pop_elem = stack.get_mut(b as usize).unwrap().pop().unwrap();
            carter_vec.push(pop_elem);
        }

        carter_vec.reverse();

        for i in carter_vec {
            stack.get_mut(c as usize).unwrap().push(i);
        }

    }

}

fn get_top_elem_from_stack(stack: &Vec<Vec<char>>) -> String {

    let mut top_elem = String::new();
    for i in stack {
        let elem = *i.get(i.len() - 1).unwrap();
        top_elem.push(elem);
    }

    top_elem

}





















