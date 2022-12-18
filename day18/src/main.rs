use std::{collections::HashSet, error::Error};

use itertools::Itertools;
use utils::read_input_file;

fn get_possible_neighbours(x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
    vec![
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut total_faces = 0;

    let mut cubes = HashSet::new();
    for cube in file_content.lines() {
        let (x, y, z) = cube
            .split(',')
            .collect_tuple()
            .ok_or("could not split cube coordinates")?;

        let x = x.parse::<i32>()?;
        let y = y.parse::<i32>()?;
        let z = z.parse::<i32>()?;

        let mut visible_cube_faces = 6;

        let possible_neighbours = get_possible_neighbours(x, y, z);
        for neighbour in possible_neighbours {
            if cubes.contains(&neighbour) {
                visible_cube_faces -= 1;
                total_faces -= 1;
            }
        }

        cubes.insert((x, y, z));
        total_faces += visible_cube_faces;
    }

    let part_1 = total_faces;

    // let &(max_x, _, _) = cubes
    //     .iter()
    //     .max_by_key(|&(x, _, _)| x)
    //     .ok_or("could not get max x")?;
    // let &(_, max_y, _) = cubes
    //     .iter()
    //     .max_by_key(|&(_, y, _)| y)
    //     .ok_or("could not get max y")?;
    // let &(_, _, max_z) = cubes
    //     .iter()
    //     .max_by_key(|&(_, _, z)| z)
    //     .ok_or("could not get max z")?;
    //
    // let mut air_pockets = HashSet::new();
    // for x in 1..=max_x {
    //     for y in 1..=max_y {
    //         for z in 1..=max_z {
    //             let possible_cube = (x, y, z);
    //             let possible_neighbours = get_possible_neighbours(x, y, z);
    //             if !cubes.contains(&possible_cube)
    //                 && possible_neighbours
    //                     .iter()
    //                     .all(|neighbour| cubes.contains(neighbour))
    //             {
    //
    //             }
    //         }
    //     }
    // }
    //
    // let part_2 = total_faces;

    println!("Part 1: {}", part_1);
    // println!("Part 2: {}", part_2);

    Ok(())
}
