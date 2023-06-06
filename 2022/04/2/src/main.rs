use std::{fs, ops::RangeInclusive};

fn main() {
    let list = fs::read_to_string("input.txt").unwrap();

    let mut counter = 0;

    for line in list.lines().map(str::trim) {
        let mut pairs = Vec::new();

        for pair in line.split(',') {
            let (start, end) = pair.split_once('-').unwrap();
            let start = start.trim().parse::<u32>().unwrap();
            let end = end.trim().parse::<u32>().unwrap();

            pairs.push(RangeInclusive::new(start, end));
        }

        let is_overlap = |x: &RangeInclusive<u32>, y: &RangeInclusive<u32>| {
            x.start() <= y.end() && y.start() <= x.end()
        };

        if is_overlap(&pairs[0], &pairs[1]) || is_overlap(&pairs[1], &pairs[0]) {
            println!("Contains! {pairs:?}");
            counter += 1;
        }
    }

    println!("{counter}");
}
