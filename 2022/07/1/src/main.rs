use std::{collections::HashMap, fs, path::PathBuf};

fn main() {
    let output = fs::read_to_string("input.txt").unwrap();

    let mut current_path = PathBuf::new();
    let mut folders: HashMap<PathBuf, u32> = HashMap::new();

    for line in output.lines().map(str::trim) {
        let mut split = line.split(' ');

        if line.starts_with('$') {
            split.next().unwrap();
            let cmd = split.next().unwrap();
            let arg = split.next();

            match (cmd, arg) {
                ("cd", Some("..")) => {
                    let parent = current_path.parent().unwrap();
                    current_path = parent.to_path_buf();
                }
                ("cd", Some(arg)) => current_path.push(arg),
                ("ls", _) => (),
                _ => panic!("Invalid command: {cmd}, {arg:?}"),
            }
        } else {
            let size = split.next().unwrap();
            split.next().unwrap();

            if let Ok(file_size) = size.parse::<u32>() {
                append_size_to_folder(&mut folders, &current_path, file_size);

                for ancestor in current_path.ancestors().skip(1) {
                    append_size_to_folder(&mut folders, &ancestor.to_path_buf(), file_size);
                }
            }
        }
    }

    let size: u32 = folders.values().filter(|x| x < &&100_000).sum();
    println!("{size}");
}

fn append_size_to_folder(folders: &mut HashMap<PathBuf, u32>, folder: &PathBuf, size: u32) {
    if !folders.contains_key(folder) {
        folders.insert(folder.clone(), 0);
    }

    *folders.get_mut(folder).unwrap() += size;
}
