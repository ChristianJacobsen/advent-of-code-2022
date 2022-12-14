use std::{error::Error, fmt::Display, thread::sleep, time::Duration, vec};

use itertools::Itertools;
use utils::read_input_file;

#[derive(Clone, PartialEq)]
enum Block {
    Air,
    Rock,
    Sand,
    SandSpawn,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Air => write!(f, "."),
            Block::Rock => write!(f, "#"),
            Block::Sand => write!(f, "o"),
            Block::SandSpawn => write!(f, "+"),
        }
    }
}

#[derive(Debug)]
struct Path((usize, usize), (usize, usize));

fn create_grid_from_path(
    rock_paths: &[Path],
    smallest_x: usize,
    largest_x: usize,
    largest_y: usize,
    with_floor: bool,
) -> Vec<Vec<Block>> {
    Vec::new()
}

fn print_grid(grid: &Vec<Vec<Block>>) {
    for row in grid {
        for entry in row {
            print!("{}", entry);
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut smallest_x = usize::MAX;
    let mut largest_x = usize::MIN;
    let mut largest_y = usize::MIN;

    let mut rock_paths = Vec::new();
    for line in file_content.lines() {
        let parts = line.split(" -> ").collect_vec();
        for window in parts.windows(2) {
            match window {
                &[start, end] => {
                    let (start_x, start_y) =
                        start.split_once(',').ok_or("could not split start")?;
                    let (end_x, end_y) = end.split_once(',').ok_or("could not split end")?;

                    let start_x = start_x.parse::<usize>()?;
                    let start_y = start_y.parse::<usize>()?;
                    let end_x = end_x.parse::<usize>()?;
                    let end_y = end_y.parse::<usize>()?;

                    if start_x < smallest_x {
                        smallest_x = start_x;
                    }

                    if end_x < smallest_x {
                        smallest_x = end_x;
                    }

                    if start_x > largest_x {
                        largest_x = start_x;
                    }

                    if end_x > largest_x {
                        largest_x = end_x;
                    }

                    if start_y > largest_y {
                        largest_y = start_y;
                    }

                    if end_y > largest_y {
                        largest_y = end_y;
                    }

                    rock_paths.push(Path((start_x, start_y), (end_x, end_y)));
                }
                _ => return Err("unexpected window".into()),
            }
        }
    }

    let width = largest_x - smallest_x + 1;
    let height = largest_y;
    let mut grid = vec![vec![Block::Air; width]; height];

    let sand_spawn_x = 500 - smallest_x;
    grid[0][sand_spawn_x] = Block::SandSpawn;

    for &Path((start_x, start_y), (end_x, end_y)) in &rock_paths {
        let y_diff = end_y as i32 - start_y as i32;

        if y_diff < 0 {
            // Up
            let x = start_x - smallest_x;
            for y in end_y - 1..start_y {
                grid[y][x] = Block::Rock;
            }
        } else if y_diff == 0 {
            // Left or Right
            let x_diff = end_x as i32 - start_x as i32;

            if x_diff < 0 {
                // Left
                let y = start_y - 1;
                for x in end_x - smallest_x..=start_x - smallest_x {
                    grid[y][x] = Block::Rock;
                }
            } else if x_diff == 0 {
                // Should never happen
                panic!("we should never have a (0, 0) diff path");
            } else {
                // Right
                let y = start_y - 1;
                for x in start_x - smallest_x..=end_x - smallest_x {
                    grid[y][x] = Block::Rock;
                }
            }
        } else {
            // Down
            let x = start_x - smallest_x;
            for y in start_y - 1..end_y {
                grid[y][x] = Block::Rock;
            }
        }
    }

    let mut sand_block_count = 0;

    loop {
        let mut sand_has_fallen_off = false;

        let mut x = sand_spawn_x;
        let mut y = 0;

        loop {
            match grid[y + 1][x] {
                Block::Air => {
                    if y == height - 2 {
                        sand_has_fallen_off = true;
                        break;
                    }

                    y += 1;
                }
                Block::Rock | Block::Sand => {
                    if x == 0 {
                        sand_has_fallen_off = true;
                        break;
                    }

                    if grid[y + 1][x - 1] == Block::Air {
                        x -= 1;
                        continue;
                    }

                    if x == width - 1 {
                        sand_has_fallen_off = true;
                        break;
                    }

                    if grid[y + 1][x + 1] == Block::Air {
                        x += 1;
                        continue;
                    }

                    grid[y][x] = Block::Sand;
                    sand_block_count += 1;
                    break;
                }
                Block::SandSpawn => panic!("should never get a SandSpawn block"),
            }
        }

        if sand_has_fallen_off {
            break;
        }
    }

    print_grid(&grid);

    let part_1_sum = sand_block_count;

    println!("Part 1: {}", part_1_sum);

    Ok(())
}
