use std::error::Error;

use utils::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut part_1_sum = 0;
    for line in file_content.lines() {
        if let Some((l, r)) = line.split_once(' ') {
            match (l, r) {
                ("A", "X") => part_1_sum += 4, // Rock vs Rock(1), Draw(3)
                ("B", "X") => part_1_sum += 1, // Paper vs Rock(1), Loss(0)
                ("C", "X") => part_1_sum += 7, // Scissors vs Rock(1), Victory(6)
                ("A", "Y") => part_1_sum += 8, // Rock vs Paper(2), Victory(6)
                ("B", "Y") => part_1_sum += 5, // Paper vs Paper(2), Draw(3)
                ("C", "Y") => part_1_sum += 2, // Scissors vs Paper(2), Loss(0)
                ("A", "Z") => part_1_sum += 3, // Rock vs Scissors(3), Loss(0)
                ("B", "Z") => part_1_sum += 9, // Paper vs Scissors(3), Victory(6)
                ("C", "Z") => part_1_sum += 6, // Scissors vs Scissors(3), Draw(3)
                _ => eprintln!("Something went wrong"),
            }
        }
    }

    let mut part_2_sum = 0;
    for line in file_content.lines() {
        if let Some((l, r)) = line.split_once(' ') {
            match (l, r) {
                ("A", "X") => part_2_sum += 3, // Rock => Loss(0), Scissors(3)
                ("B", "X") => part_2_sum += 1, // Paper => Loss(0), Rock(1)
                ("C", "X") => part_2_sum += 2, // Scissors => Loss(0), Paper(2)
                ("A", "Y") => part_2_sum += 4, // Rock => Draw(3), Rock(1)
                ("B", "Y") => part_2_sum += 5, // Paper => Draw(3), Paper(2)
                ("C", "Y") => part_2_sum += 6, // Scissors => Draw(3), Scissors(3)
                ("A", "Z") => part_2_sum += 8, // Rock => Victory(6), Paper(2)
                ("B", "Z") => part_2_sum += 9, // Paper => Victory(6), Scissors(3)
                ("C", "Z") => part_2_sum += 7, // Scissors => Victory(6), Rock(1)
                _ => eprintln!("Something went wrong"),
            }
        }
    }

    println!("Part 1: {}", part_1_sum);
    println!("Part 2: {}", part_2_sum);

    Ok(())
}
