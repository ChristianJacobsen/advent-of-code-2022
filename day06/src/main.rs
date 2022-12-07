use std::error::Error;

use itertools::Itertools;
use utils::read_input_file;

fn last_index_of_n_unique_chars(slice: &[char], n: usize) -> Option<usize> {
    slice
        .windows(n)
        .position(|window| window.iter().all_unique())
        .and_then(|pos| Some(pos + n))
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
