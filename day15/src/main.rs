use std::{collections::HashSet, error::Error};

use itertools::Itertools;
use utils::read_input_file;

fn manhattan_distance((a_x, a_y): &(i32, i32), (b_x, b_y): &(i32, i32)) -> i32 {
    (b_x - a_x).abs() + (b_y - a_y).abs()
}

fn is_valid(pos: &(i32, i32), sensors_and_distances: &HashSet<((i32, i32), i32)>) -> bool {
    for &(sensor, distance) in sensors_and_distances {
        if manhattan_distance(&sensor, pos) <= distance {
            return false;
        }
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut sensors_with_beacon_distances = HashSet::new();
    let mut beacons = HashSet::new();
    for line in file_content.lines() {
        let parts = line.split_whitespace().collect_vec();

        let sensor_x = parts[2][2..parts[2].len() - 1].parse::<i32>()?;
        let sensor_y = parts[3][2..parts[3].len() - 1].parse::<i32>()?;

        let beacon_x = parts[8][2..parts[8].len() - 1].parse::<i32>()?;
        let beacon_y = parts[9][2..].parse::<i32>()?;

        let distance = manhattan_distance(&(sensor_x, sensor_y), &(beacon_x, beacon_y));

        sensors_with_beacon_distances.insert(((sensor_x, sensor_y), distance));
        beacons.insert((beacon_x, beacon_y));
    }

    let mut invalid_positions = HashSet::new();

    let y = 2_000_000;
    for x in -5_000_000..=5_000_000 {
        let pos = (x, y);

        if !is_valid(&pos, &sensors_with_beacon_distances)
            && !beacons.contains(&pos)
            && !invalid_positions.contains(&pos)
        {
            invalid_positions.insert(pos);
        }
    }

    let part_1_sum = invalid_positions.len();


    let mut beacon_frequency = None;

    const DIRECTIONS: [(i32, i32); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
    for &((sensor_x, sensor_y), distance) in &sensors_with_beacon_distances {
        if beacon_frequency.is_some() {
            break;
        }

        for x_distance in 0..=distance + 1 {
            if beacon_frequency.is_some() {
                break;
            }

            let y_distance = (distance + 1) - x_distance;

            for (x_direction, y_direction) in DIRECTIONS {
                let x = sensor_x + (x_distance * x_direction);
                let y = sensor_y + (y_distance * y_direction);

                if !((0..=4_000_000).contains(&x) && (0..=4_000_000).contains(&y)) {
                    continue;
                }

                if is_valid(&(x, y), &sensors_with_beacon_distances) {
                    beacon_frequency = Some(x as i64 * 4_000_000 + y as i64);
                    break;
                }
            }
        }
    }

    let part_2 = beacon_frequency.ok_or("could not calculate beacon frequency")?;

    println!("Part 1: {}", part_1_sum);
    println!("Part 2: {}", part_2);

    Ok(())
}
