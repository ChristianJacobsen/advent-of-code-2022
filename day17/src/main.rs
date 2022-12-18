use std::{collections::HashSet, error::Error, str::FromStr};

use utils::read_input_file;

#[derive(Debug)]
enum GasFlowDirection {
    Left,
    Right,
}

impl FromStr for GasFlowDirection {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "<" {
            Ok(Self::Left)
        } else if s == ">" {
            Ok(Self::Right)
        } else {
            Err(format!("invalid direction '{}'", s).into())
        }
    }
}

fn get_rock(rock_number: i64, y: i64) -> HashSet<(i64, i64)> {
    match rock_number % 5 {
        0 => HashSet::from([(2, y), (3, y), (4, y), (5, y)]),
        1 => HashSet::from([(3, y + 2), (2, y + 1), (3, y + 1), (4, y + 1), (3, y)]),
        2 => HashSet::from([(4, y + 2), (4, y + 1), (2, y), (3, y), (4, y)]),
        3 => HashSet::from([(2, y + 3), (2, y + 2), (2, y + 1), (2, y)]),
        4 => HashSet::from([(2, y + 1), (3, y + 1), (2, y), (3, y)]),
        _ => panic!("modulo in shambles"),
    }
}

fn move_left(mut rock: HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    if rock.iter().any(|&(x, _)| x == 0) {
        return rock;
    }
    rock.drain().map(|(x, y)| (x - 1, y)).collect()
}

fn move_right(mut rock: HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    if rock.iter().any(|&(x, _)| x == 6) {
        return rock;
    }
    rock.drain().map(|(x, y)| (x + 1, y)).collect()
}

fn move_down(mut rock: HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    rock.drain().map(|(x, y)| (x, y - 1)).collect()
}

fn move_up(mut rock: HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    rock.drain().map(|(x, y)| (x, y + 1)).collect()
}

fn keep_top_n_layers(
    mut rock_positions: HashSet<(i64, i64)>,
    tallest_rock_position: i64,
    n: i64,
) -> HashSet<(i64, i64)> {
    let mut to_remove = Vec::new();
    for rock in &rock_positions {
        let &(_, y) = rock;
        if (y as i64) < (tallest_rock_position as i64 - n as i64) {
            to_remove.push(*rock);
        }
    }
    for rock in to_remove {
        rock_positions.remove(&rock);
    }
    rock_positions
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut gas_flow_directions = Vec::new();
    for line in file_content.lines() {
        for direction in line.split("").filter(|&direction| !direction.is_empty()) {
            let direction = direction.parse::<GasFlowDirection>()?;
            gas_flow_directions.push(direction);
        }
    }

    let mut directions = gas_flow_directions.iter().cycle();
    let mut rock_positions = HashSet::new();
    for x in 0..7 {
        rock_positions.insert((x, 0));
    }
    let mut tallest_rock_position = 0;

    for rock_number in 0..2022 {
        let mut rock = get_rock(rock_number, tallest_rock_position + 4);

        rock_positions = keep_top_n_layers(rock_positions, tallest_rock_position, 50);

        loop {
            let direction = directions.next().ok_or("could not get next direction")?;

            match direction {
                GasFlowDirection::Left => {
                    rock = move_left(rock);
                    if rock.intersection(&rock_positions).count() > 0 {
                        rock = move_right(rock);
                    }
                }
                GasFlowDirection::Right => {
                    rock = move_right(rock);
                    if rock.intersection(&rock_positions).count() > 0 {
                        rock = move_left(rock);
                    }
                }
            }

            rock = move_down(rock);
            if rock.intersection(&rock_positions).count() > 0 {
                rock = move_up(rock);
                rock_positions.extend(rock);
                tallest_rock_position = rock_positions
                    .iter()
                    .map(|&(_, y)| y)
                    .max()
                    .ok_or("could not get tallest rock")?;
                break;
            }
        }
    }

    let part_1 = tallest_rock_position;

    println!("Part 1: {}", part_1);

    Ok(())
}
