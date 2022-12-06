use std::{error::Error, collections::HashSet};

use itertools::Itertools;
use utils::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;
    let chars = file_content.chars().collect_vec();

    let mut part_1_index = 0;
    for (i, w) in chars.windows(4).enumerate() {
        let set: HashSet<_> = w.iter().collect();
        if set.len() == 4 {
            part_1_index = i + 4;
            break;
        }
    }

    let mut part_2_index = 0;
    for (i, w) in chars.windows(14).enumerate() {
        let set: HashSet<_> = w.iter().collect();
        if set.len() == 14 {
            part_2_index = i + 14;
            break;
        }
    }

    println!("Part 1: {}", part_1_index);
    println!("Part 2: {}", part_2_index);

    Ok(())
}
