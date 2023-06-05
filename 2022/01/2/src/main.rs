use std::fs;

fn main() {
    let list = fs::read_to_string("input.txt").unwrap();

    let mut elves: Vec<i32> = vec![0];
    let mut index = 0;

    for line in list.lines().map(str::trim) {
        if line.is_empty() {
            index += 1;
            elves.push(0);
            continue;
        }

        let new_callories: i32 = line.parse().unwrap();
        elves[index] += new_callories;
    }

    elves.sort_unstable();
    elves.reverse();

    println!("{}", elves[0..3].iter().sum::<i32>());
}
