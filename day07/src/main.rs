use std::{collections::HashMap, error::Error, path::PathBuf};

use itertools::Itertools;
use utils::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut dirs = HashMap::from([(PathBuf::from("/"), (0, Vec::new()))]);
    let mut current_path = PathBuf::from("/");

    for command_or_output in file_content.lines() {
        let parts = command_or_output.split_whitespace().collect_vec();
        match parts.as_slice() {
            ["$", "cd", path] => {
                match *path {
                    "/" => current_path = PathBuf::from("/"),
                    ".." => {
                        current_path.pop();
                        ()
                    }
                    path => current_path.push(path),
                };
            }
            ["$", "ls"] => {}
            ["dir", name] => {
                let mut dir_path = current_path.clone();
                dir_path.push(name);
                dirs.insert(dir_path, (0, Vec::new()));
            }
            [size, name] => {
                let current_dir = dirs
                    .get_mut(&current_path)
                    .ok_or("could not get current dir")?;
                let size = size.parse::<u32>()?;
                current_dir.0 += size;
                current_dir.1.push((size, *name));
            }
            _ => return Err(format!("could not match line: {}", command_or_output).into()),
        }
    }

    let mut dir_sizes: HashMap<PathBuf, u32> = HashMap::new();
    for (dir, (size, _)) in dirs {
        let mut path = PathBuf::new();
        for part in dir.iter() {
            path.push(part);
            if let Some(current_size) = dir_sizes.get_mut(&path) {
                *current_size += size;
            } else {
                dir_sizes.insert(path.clone(), size);
            }
        }
    }

    let part_1_sum = dir_sizes.iter().fold(0, |mut acc, (_, &size)| {
        if size <= 100000 {
            acc += size
        };
        acc
    });

    let root_size = dir_sizes
        .get(&PathBuf::from("/"))
        .ok_or("could not get root")?;
    let current_unused = 70000000 - root_size;
    let required_size = 30000000 - current_unused;
    let (_, part_2_sum) = dir_sizes
        .iter()
        .sorted_by(|(_, a), (_, b)| a.cmp(b))
        .find(|(_, &size)| size >= required_size)
        .ok_or("could not find smallest dir")?;

    println!("Part 1: {}", part_1_sum);
    println!("Part 2: {}", part_2_sum);

    Ok(())
}
