use std::fs;

#[allow(clippy::indexing_slicing)]
#[allow(clippy::needless_range_loop)]
fn main() {
    let map = fs::read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<u32>> = map
        .lines()
        .map(str::trim)
        .map(|line| line.chars().filter_map(|x| x.to_digit(10)).collect())
        .collect();

    let mut highest_score = 0;

    for line_index in 0..grid.len() {
        for tree_index in 0..grid[line_index].len() {
            // if line_index != 1 || tree_index != 2 {
            //     continue;
            // }

            let tree = grid[line_index][tree_index];

            let (left_trees, right_trees) = grid[line_index].split_at(tree_index);
            let right_trees = &right_trees[1..];

            // println!("{:?}", grid[line_index]);
            // println!("{left_trees:?}, {right_trees:?}");

            let left_visible = count(left_trees.iter().rev(), tree);
            let right_visible = count(right_trees.iter(), tree);

            // println!("left: {left_visible}, right: {right_visible}");

            let vertical_line: Vec<u32> = grid.iter().map(|x| x[tree_index]).collect();

            // println!("{vertical_line:?}");

            let (up_trees, down_trees) = vertical_line.split_at(line_index);
            let down_trees = &down_trees[1..];

            // println!("up: {up_trees:?}, down: {down_trees:?}");

            let up_visible = count(up_trees.iter().rev(), tree);
            let down_visible = count(down_trees.iter(), tree);

            // println!("1st: {up_visible} {left_visible} {right_visible} {down_visible}");
            // println!("2st: {up_visible} {left_visible} {down_visible} {right_visible}");

            let score = up_visible * left_visible * right_visible * down_visible;

            if score > highest_score {
                highest_score = score;
            }
        }
    }

    println!("Highest score possible is {highest_score}");
}

fn count<'a>(iter: impl Iterator<Item = &'a u32>, tree: u32) -> u32 {
    let mut visible = 0;

    for d_tree in iter {
        visible += 1;

        if *d_tree >= tree {
            break;
        }
    }

    visible
}
