use std::fs;

fn main() {

    let raw_file = fs::read_to_string("/home/arhant/aoc/rust/aoc8/src/file.txt").unwrap();

    let mut grid: Vec<Vec<i32>> = Vec::new();

    let mut total_trees: i32 = 0;
    let mut scenic_score: i32 = 1;

    for line in raw_file.lines() {
        let mut tree_vec: Vec<i32> = Vec::new();
        for num in line.chars() {
            tree_vec.push(num.to_digit(10).unwrap() as i32);
        }
        grid.push(tree_vec);
    }

    for (row_index, row) in grid.iter().enumerate() {
        if row_index != 0 && row_index != (grid.len() - 1) {
            for (elem_index, _) in row.iter().enumerate() {
                if elem_index != 0 && elem_index != (row.len() - 1) {

                    let fix_tree = grid.get(row_index).unwrap().get(elem_index).unwrap();

                    let mut is_visible: bool = false;

                    if !is_visible {
                        // <-
                        for i in (0..elem_index).rev() {
                            let vary_tree = grid.get(row_index).unwrap().get(i).unwrap();

                            if fix_tree <= vary_tree {
                                is_visible = false;
                                break;
                            }

                            if fix_tree > vary_tree {
                                is_visible = true;
                            }


                        }

                        if is_visible {
                            total_trees += 1;
                        }

                    }

                    if !is_visible {
                        // ->
                        for i in (elem_index + 1)..row.len() {
                            let vary_tree = grid.get(row_index).unwrap().get(i).unwrap();

                            if fix_tree <= vary_tree {
                                is_visible = false;
                                break;
                            }

                            if fix_tree > vary_tree {
                                is_visible = true;
                            }

                        }

                        if is_visible {
                            total_trees += 1;
                        }

                    }

                    if !is_visible {
                        // ^
                        for i in (0..row_index).rev() {
                            let vary_tree = grid.get(i).unwrap().get(elem_index).unwrap();

                            if fix_tree <= vary_tree {
                                is_visible = false;
                                break;
                            }

                            if fix_tree > vary_tree {
                                is_visible = true;
                            }
                        }

                        if is_visible {
                            total_trees += 1;
                        }

                    }

                    if !is_visible {
                        // V
                        for i in (row_index + 1)..(grid.len()) {
                            let vary_tree = grid.get(i).unwrap().get(elem_index).unwrap();

                            if fix_tree <= vary_tree {
                                is_visible = false;
                                break;
                            }

                            if fix_tree > vary_tree {
                                is_visible = true;
                            }
                        }

                        if is_visible {
                            total_trees += 1;
                        }

                    }

                }
            }
        }
    }

    total_trees += 2 * ((grid.len() as i32) - 2);
    total_trees += 2 * (grid.first().unwrap().len() as i32);

    println!("Part 1 -> {:?}", total_trees);

    for (row_index, row) in grid.iter().enumerate() {
        if row_index != 0 && row_index != (grid.len() - 1) {
            for (elem_index, _) in row.iter().enumerate() {
                if elem_index != 0 && elem_index != (row.len() - 1) {

                    let fix_tree = grid.get(row_index).unwrap().get(elem_index).unwrap();

                    let mut num_tree: i32 = 0;
                    let mut scenic_score_vec: Vec<i32> = Vec::new();

                    // <-
                    for i in (0..elem_index).rev() {
                        let vary_tree = grid.get(row_index).unwrap().get(i).unwrap();

                        if fix_tree <= vary_tree {
                            num_tree += 1;
                            break;
                        }

                        if fix_tree > vary_tree {
                            num_tree += 1;
                        }

                    }

                    scenic_score_vec.push(num_tree);


                    num_tree = 0;

                    // ->
                    for i in (elem_index + 1)..row.len() {
                        let vary_tree = grid.get(row_index).unwrap().get(i).unwrap();

                        if fix_tree <= vary_tree {
                            num_tree += 1;
                            break;
                        }

                        if fix_tree > vary_tree {
                            num_tree += 1;
                        }

                    }

                    scenic_score_vec.push(num_tree);

                    num_tree = 0;

                    // ^
                    for i in (0..row_index).rev() {
                        let vary_tree = grid.get(i).unwrap().get(elem_index).unwrap();

                        if fix_tree <= vary_tree {
                            num_tree += 1;
                            break;
                        }

                        if fix_tree > vary_tree {
                            num_tree += 1;
                        }
                    }

                    scenic_score_vec.push(num_tree);

                    num_tree = 0;

                    // V
                    for i in (row_index + 1)..(grid.len()) {
                        let vary_tree = grid.get(i).unwrap().get(elem_index).unwrap();

                        if fix_tree <= vary_tree {
                            num_tree += 1;
                            break;
                        }

                        if fix_tree > vary_tree {
                            num_tree += 1;
                        }
                    }

                    scenic_score_vec.push(num_tree);

                    let mut mul: i32 = 1;

                    for i in &scenic_score_vec {
                        mul *= i;
                    }

                    if mul > scenic_score {
                        scenic_score = mul;
                    }

                }
            }
        }
    }

    println!("Part 2 -> {:?}", scenic_score);

}
