use std::error::Error;

use itertools::Itertools;
use pathfinding::prelude::bfs;
use utils::read_input_file;

fn get_viable_positions(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let current_elevation = grid[y][x];

    let width = grid.first().expect("should have at least one row").len();
    let height = grid.len();

    let mut viable_paths = Vec::new();

    // Left
    if x > 0 {
        let diff = diff_char(grid[y][x - 1], current_elevation);
        if diff <= 1 {
            viable_paths.push((x - 1, y));
        }
    }

    // Up
    if y > 0 {
        let diff = diff_char(grid[y - 1][x], current_elevation);
        if diff <= 1 {
            viable_paths.push((x, y - 1));
        }
    }

    // Right
    if x < width - 1 {
        let diff = diff_char(grid[y][x + 1], current_elevation);
        if diff <= 1 {
            viable_paths.push((x + 1, y));
        }
    }

    // Down
    if y < height - 1 {
        let diff = diff_char(grid[y + 1][x], current_elevation);
        if diff <= 1 {
            viable_paths.push((x, y + 1));
        }
    }

    viable_paths
}

fn diff_char(a: char, b: char) -> i8 {
    a as i8 - b as i8
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut grid = Vec::new();
    for grid_row in file_content.lines() {
        grid.push(grid_row.chars().collect_vec());
    }

    let mut start_position = (0, 0);
    let mut destination = (0, 0);
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, c) in row.iter_mut().enumerate() {
            if *c == 'S' {
                *c = 'a';
                start_position = (j, i);
            } else if *c == 'E' {
                *c = 'z';
                destination = (j, i);
            }
        }
    }

    let path = bfs(
        &start_position,
        |&(x, y)| get_viable_positions(&grid, x, y),
        |&p| p == destination,
    )
    .ok_or("could not calculate shortest path")?;

    let part_1_sum = path.len() - 1;

    let mut lowest_elevation_positions = Vec::new();
    for (i, grid_row) in grid.iter().enumerate() {
        for (j, &c) in grid_row.iter().enumerate() {
            if c == 'a' {
                lowest_elevation_positions.push((j, i));
            }
        }
    }

    let mut paths_from_lowest_elevations = Vec::new();
    for position in lowest_elevation_positions {
        if let Some(path) = bfs(
            &position,
            |&(x, y)| get_viable_positions(&grid, x, y),
            |&p| p == destination,
        ) {
            paths_from_lowest_elevations.push(path.len() - 1);
        }
    }

    let part_2_sum = paths_from_lowest_elevations
        .iter()
        .min()
        .ok_or("could not get shortest path")?;

    println!("Part 1: {}", part_1_sum);
    println!("Part 2: {}", part_2_sum);

    Ok(())
}
