use std::{collections::HashSet, error::Error, str::FromStr};

use utils::read_input_file;

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "L" => Ok(Direction::Left),
            "D" => Ok(Direction::Down),
            "R" => Ok(Direction::Right),
            _ => Err("invalid direction".into()),
        }
    }
}

fn get_new_tail_position(head_position: &(i32, i32), tail_position: &(i32, i32)) -> (i32, i32) {
    let (head_x, head_y) = head_position;
    let (mut tail_x, mut tail_y) = tail_position;

    let x_distance = head_x - tail_x;
    let y_distance = head_y - tail_y;

    match (x_distance, y_distance) {
        (1, 2) | (2, 1) | (2, 2) => {
            tail_x += 1;
            tail_y += 1;
        }
        (0, 2) => {
            tail_y += 1;
        }
        (-1, 2) | (-2, 1) | (-2, 2) => {
            tail_x -= 1;
            tail_y += 1;
        }
        (-2, 0) => {
            tail_x -= 1;
        }
        (-1, -2) | (-2, -1) | (-2, -2) => {
            tail_x -= 1;
            tail_y -= 1;
        }
        (0, -2) => {
            tail_y -= 1;
        }
        (1, -2) | (2, -1) | (2, -2) => {
            tail_x += 1;
            tail_y -= 1;
        }
        (2, 0) => {
            tail_x += 1;
        }
        _ => {}
    }

    (tail_x, tail_y)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    let mut visited_tail_positions = HashSet::from([(0, 0)]);

    for command in file_content.lines() {
        let (direction, steps) = command
            .split_once(' ')
            .ok_or("could not split into direction and steps")?;

        let direction = direction.parse::<Direction>()?;
        let steps = steps.parse::<u32>()?;

        for _ in 0..steps {
            let (mut head_x, mut head_y) = head_position;

            match direction {
                Direction::Up => head_y += 1,
                Direction::Left => head_x -= 1,
                Direction::Down => head_y -= 1,
                Direction::Right => head_x += 1,
            }

            let new_head_position = (head_x, head_y);
            let new_tail_position = get_new_tail_position(&new_head_position, &tail_position);

            if new_tail_position != tail_position {
                visited_tail_positions.insert(new_tail_position);
            }

            head_position = new_head_position;
            tail_position = new_tail_position;
        }
    }

    let part_1_sum = visited_tail_positions.len();

    let mut positions = vec![(0, 0); 10];
    let mut visited_last_tail_positions = HashSet::from([(0, 0)]);

    for command in file_content.lines() {
        let (direction, steps) = command
            .split_once(' ')
            .ok_or("could not split into direction and steps")?;

        let direction = direction.parse::<Direction>()?;
        let steps = steps.parse::<u32>()?;

        for _ in 0..steps {
            for i in 0..positions.len() - 1 {
                let (mut head_x, mut head_y) = positions[i];

                if i == 0 {
                    match direction {
                        Direction::Up => head_y += 1,
                        Direction::Left => head_x -= 1,
                        Direction::Down => head_y -= 1,
                        Direction::Right => head_x += 1,
                    }
                }

                let new_head_position = (head_x, head_y);

                let old_tail_position = positions[i + 1];
                let new_tail_position =
                    get_new_tail_position(&new_head_position, &old_tail_position);

                let is_last_tail = i + 2 == positions.len();
                if is_last_tail && new_tail_position != old_tail_position {
                    visited_last_tail_positions.insert(new_tail_position);
                }

                positions[i] = new_head_position;
                positions[i + 1] = new_tail_position;
            }
        }
    }

    let part_2_sum = visited_last_tail_positions.len();

    println!("Part 1: {}", part_1_sum);
    println!("Part 2: {}", part_2_sum);

    Ok(())
}
