use std::fs;

#[allow(clippy::indexing_slicing)]
fn main() {
    let map = fs::read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<u32>> = map
        .lines()
        .map(str::trim)
        .map(|line| line.chars().filter_map(|x| x.to_digit(10)).collect())
        .collect();

    let mut visible = 0;

    let last_line = grid.len() - 1;

    for line in 0..grid.len() {
        if line == 0 || line == last_line {
            visible += grid[line].len();
            continue;
        }

        visible += 2;
        for tree_index in 1..grid[line].len() - 1 {
            let current_tree: u32 = grid[line][tree_index];

            let down_visible = grid
                .iter()
                .skip(line)
                .skip(1)
                .map(|line| line[tree_index])
                .all(|tree| current_tree > tree);

            let up_visible = grid
                .iter()
                .rev()
                .skip(grid.len() - line)
                .map(|line| line[tree_index])
                .all(|tree| current_tree > tree);

            let right_visible = grid[line]
                .iter()
                .skip(tree_index)
                .skip(1)
                .all(|tree| current_tree > *tree);

            let left_visible = grid[line]
                .iter()
                .rev()
                .skip(grid[line].len() - tree_index)
                .all(|tree| current_tree > *tree);

            if down_visible || up_visible || right_visible || left_visible {
                println!("{line} {tree_index}: {down_visible}, {up_visible}, {right_visible}, {left_visible}");
                visible += 1;
            }
        }
    }

    println!("{visible}");
}
