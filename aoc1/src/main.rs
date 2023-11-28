fn main() {

    let raw_file = std::fs::read_to_string("/home/arhant/aoc/rust/aoc1/src/file.txt").unwrap();
    let int_file: Vec<u32> = raw_file.lines().map(|s| s.parse::<u32>().unwrap_or(0)).collect();

    let mut all_cal: Vec<u32> = Vec::new();
    let mut curr_cal = 0;

    for i in int_file {

        if i == 0 {
            all_cal.push(curr_cal);
            curr_cal = 0;
        } else {
            curr_cal += i;
        }

    }

    let Some((_, max_cal)) = all_cal.iter().enumerate().max_by_key(|&(_, t)| { t }) else { todo!() };

    println!("Part 1 -> {:?}", max_cal);

    all_cal.sort();

    let top_three_max_call = all_cal[all_cal.len() - 1] + all_cal[all_cal.len() - 2] + all_cal[all_cal.len() - 3];

    println!("Part 2 -> {:?}", top_three_max_call);
    
}
