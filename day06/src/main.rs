use std::{collections::HashSet, error::Error};

use itertools::Itertools;
use utils::read_input_file;

fn last_index_of_n_unique_chars(slice: &[char], n: usize) -> Option<usize> {
    for (i, w) in slice.windows(n).enumerate() {
        let set: HashSet<_> = w.iter().collect();
        if set.len() == n {
            return Some(i + n);
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;
    let chars = file_content.chars().collect_vec();

    let part_1_index =
        last_index_of_n_unique_chars(&chars, 4).ok_or("could not calculate part 1")?;

    let part_2_index =
        last_index_of_n_unique_chars(&chars, 14).ok_or("could not calculate part 2")?;

    println!("Part 1: {}", part_1_index);
    println!("Part 2: {}", part_2_index);

    Ok(())
}
