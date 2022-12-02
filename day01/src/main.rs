use std::error::Error;

use utils::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut loads: Vec<_> = file_content
        .split("\n\n")
        .map(|load| {
            load.lines().fold(0, |mut current, amount| {
                current += amount.parse::<u32>().unwrap();
                current
            })
        })
        .collect();
    loads.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", loads.first().unwrap());
    println!("Part 2: {}", loads.iter().take(3).sum::<u32>());

    Ok(())
}
