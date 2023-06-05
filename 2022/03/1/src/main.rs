use std::fs;

fn main() {
    let list = fs::read_to_string("input.txt").unwrap();

    let mut priority: u32 = 0;

    for rucksack in list.lines().map(str::trim) {
        // Compartments
        let (first, second) = rucksack.split_at(rucksack.len() / 2);

        let first_items: Vec<char> = first.chars().collect();

        let mut duplicates: Vec<char> =
            second.chars().filter(|x| first_items.contains(x)).collect();

        duplicates.dedup();

        for duplicate in duplicates {
            if duplicate.is_uppercase() {
                priority += 26;
            }

            priority += duplicate.to_ascii_lowercase() as u32 - 96;
        }
    }

    println!("{priority}");
}
