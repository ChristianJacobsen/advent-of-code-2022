use std::{
    cmp::{max, Ordering},
    collections::HashSet,
    error::Error,
};

use itertools::Itertools;
use utils::read_input_file;

fn signum(val: i32) -> i32 {
    match val.cmp(&0) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut rock_and_sand_positions = HashSet::new();

    for line in file_content.lines() {
        let mut prev = None;

        for point in line.split(" -> ") {
            let (x, y) = point.split_once(',').ok_or("could not split point")?;

            let x = x.parse::<i32>()?;
            let y = y.parse::<i32>()?;

            if let Some((prev_x, prev_y)) = prev {
                let x_diff: i32 = x - prev_x;
                let y_diff: i32 = y - prev_y;

                let length = max(x_diff.abs(), y_diff.abs());

                for i in 0..length + 1 {
                    let x = prev_x + i * signum(x_diff);
                    let y = prev_y + i * signum(y_diff);

                    rock_and_sand_positions.insert((x, y));
                }
            }

            prev = Some((x, y))
        }
    }

    let (_, max_y) = rock_and_sand_positions
        .iter()
        .max_by_key(|(_, y)| y)
        .ok_or("could not get max y")?;
    let floor_y = max_y + 2;

    let (&(min_x, _), &(max_x, _)) = rock_and_sand_positions
        .iter()
        .minmax_by_key(|(x, _)| x)
        .into_option()
        .ok_or("could not get min and max x")?;

    let min_x = min_x - 5000;
    let max_x = max_x + 5000;

    for x in min_x..max_x {
        rock_and_sand_positions.insert((x, floor_y));
    }

    let mut part_1_sum = 0;
    let mut part_2_sum = 0;

    let sand_spawner = (500, 0);

    for i in 0..i32::MAX {
        let mut sand_block = sand_spawner;

        loop {
            let (sand_block_x, sand_block_y) = sand_block;

            if sand_block_y + 1 >= floor_y && part_1_sum == 0 {
                part_1_sum = i;
            }

            if !rock_and_sand_positions.contains(&(sand_block_x, sand_block_y + 1)) {
                sand_block = (sand_block_x, sand_block_y + 1);
            } else if !rock_and_sand_positions.contains(&(sand_block_x - 1, sand_block_y + 1)) {
                sand_block = (sand_block_x - 1, sand_block_y + 1);
            } else if !rock_and_sand_positions.contains(&(sand_block_x + 1, sand_block_y + 1)) {
                sand_block = (sand_block_x + 1, sand_block_y + 1);
            } else {
                break;
            }
        }

        if sand_block == sand_spawner {
            part_2_sum = i + 1;
            break;
        }

        rock_and_sand_positions.insert(sand_block);
    }

    println!("Part 1: {}", part_1_sum);
    println!("Part 2: {}", part_2_sum);

    Ok(())
}
