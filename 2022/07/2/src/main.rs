use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

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

    let total_used_space = folders[Path::new("/")];
    let unused_space = 70_000_000 - total_used_space;
    let required_space = 30_000_000 - unused_space;

    let mut suitable_folders: Vec<&u32> =
        folders.values().filter(|x| **x >= required_space).collect();

    suitable_folders.sort_unstable();

    println!("{}", suitable_folders.first().unwrap());
}

fn append_size_to_folder(folders: &mut HashMap<PathBuf, u32>, folder: &PathBuf, size: u32) {
    if !folders.contains_key(folder) {
        folders.insert(folder.clone(), 0);
    }

    *folders.get_mut(folder).unwrap() += size;
}
