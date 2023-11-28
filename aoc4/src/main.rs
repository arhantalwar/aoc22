use std::fs;

fn main() {

    let raw_file = fs::read_to_string("/home/arhant/aoc/rust/aoc4/src/file.txt").unwrap();
    let mut section_vec: Vec<i32> = Vec::new();

    raw_file.lines().for_each(|x| {
        x.split(",").for_each(|y| { 
            y.split("-").for_each(|z| {
                section_vec.push(z.parse::<i32>().unwrap());
            })
        });
    });

    let inner_iter_chunk = section_vec.chunks(4);
    let mut total_num = 0;
    let mut total_num_no_lap = 0;

    for elem in inner_iter_chunk {

        let a1 = elem[0];
        let a2 = elem[1];

        let b1 = elem[2];
        let b2 = elem[3];

        if a1 <= b1 && a2 >= b2 {
            total_num += 1;
        } else if a1 >= b1 && a2 <= b2 {
            total_num += 1;
        }

        if a1 <= b1 && b1 <= a2 {
            total_num_no_lap += 1;
        } else if a1 <= b2 && b2 <= a2 {
            total_num_no_lap += 1;
        } else if a1 <= b1 && a2 >= b2 {
            total_num_no_lap += 1;
        } else if a1 >= b1 && a2 <= b2 {
            total_num_no_lap += 1;
        }

    }

    println!("Part 1 -> {:?}", total_num);
    println!("Part 2 -> {:?}", total_num_no_lap);

}
