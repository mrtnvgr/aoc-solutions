use std::fs;

fn main() {
    let list = fs::read_to_string("input.txt").unwrap();

    let mut priority: u32 = 0;
    let mut group_buffer: Vec<&str> = Vec::new();

    for (i, line) in list.lines().map(str::trim).enumerate() {
        group_buffer.push(line);

        if (i + 1) % 3 != 0 {
            continue;
        }

        let mut duplicates: Vec<char> = Vec::new();

        for char in group_buffer[0].chars() {
            if group_buffer[1].contains(char)
                && group_buffer[2].contains(char)
                && !duplicates.contains(&char)
            {
                duplicates.push(char);
            }
        }

        for duplicate in duplicates {
            if duplicate.is_uppercase() {
                priority += 26;
            }

            priority += duplicate.to_ascii_lowercase() as u32 - 96;
        }

        group_buffer.clear();
    }

    println!("{priority}");
}
