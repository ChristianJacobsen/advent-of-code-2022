use std::{
    collections::{hash_map::RandomState, hash_set::Intersection, HashSet},
    error::Error,
};

use itertools::Itertools;
use utils::read_input_file;

fn calculate_char_intersection_sum(
    intersection: Intersection<&char, RandomState>,
) -> Result<i32, Box<dyn Error>> {
    let mut sum: i32 = 0;
    for char in intersection {
        match char {
            'A'..='Z' => {
                sum += ((**char as u8) - ('A' as u8) + 27) as i32;
            }
            'a'..='z' => {
                sum += ((**char as u8) - ('a' as u8) + 1) as i32;
            }
            _ => return Err(format!("found illegal character {}", char).into()),
        }
    }
    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut sum_part_1 = 0;
    for line in file_content.lines() {
        let chars = line.chars().collect_vec();
        let (comp_1, comp_2) = chars.split_at(chars.len() / 2);
        let comp_1: HashSet<_> = comp_1.into_iter().collect();
        let comp_2: HashSet<_> = comp_2.into_iter().collect();
        let intersection = comp_1.intersection(&comp_2);
        sum_part_1 += calculate_char_intersection_sum(intersection)?;
    }

    let mut sum_part_2 = 0;
    for team_members in file_content.lines().collect_vec().chunks(3) {
        let comp_1: HashSet<_> = team_members[0].chars().collect();
        let comp_2: HashSet<_> = team_members[1].chars().collect();
        let comp_3: HashSet<_> = team_members[2].chars().collect();
        let comp_1_2_intersection: HashSet<_> = comp_1.intersection(&comp_2).collect();
        let comp_2_3_intersection: HashSet<_> = comp_2.intersection(&comp_3).collect();
        let intersection = comp_1_2_intersection.intersection(&comp_2_3_intersection);
        sum_part_2 += calculate_char_intersection_sum(intersection)?;
    }

    println!("Part 1: {}", sum_part_1);
    println!("Part 2: {}", sum_part_2);

    Ok(())
}
