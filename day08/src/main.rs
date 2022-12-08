use std::error::Error;

use itertools::Itertools;
use utils::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut trees = Vec::new();
    for tree_row in file_content.lines() {
        trees.push(
            tree_row
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec(),
        );
    }

    let width = trees.first().unwrap().len();
    let height = trees.len();

    let mut visible_trees = 2 * width + 2 * (height - 2);

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let this_tree = trees[i][j];

            let mut trees_top = Vec::new();
            for tree_row in &trees[0..i] {
                trees_top.push(tree_row[j]);
            }

            if trees_top.iter().all(|&tree| this_tree > tree) {
                visible_trees += 1;
                continue;
            }

            let mut trees_left = Vec::new();
            for &tree in &trees[i][0..j] {
                trees_left.push(tree);
            }

            if trees_left.iter().all(|&tree| this_tree > tree) {
                visible_trees += 1;
                continue;
            }

            let mut trees_bottom = Vec::new();
            for tree_row in &trees[i + 1..height] {
                trees_bottom.push(tree_row[j]);
            }

            if trees_bottom.iter().all(|&tree| this_tree > tree) {
                visible_trees += 1;
                continue;
            }

            let mut trees_right = Vec::new();
            for &tree in &trees[i][j + 1..width] {
                trees_right.push(tree);
            }

            if trees_right.iter().all(|&tree| this_tree > tree) {
                visible_trees += 1;
                continue;
            }
        }
    }

    let part_1_sum = visible_trees;

    let mut visibility_scores = Vec::new();
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let this_tree = trees[i][j];

            let mut visibility_top = 0;
            for tree_row in trees[0..i].iter().rev() {
                let tree = tree_row[j];

                visibility_top += 1;

                if this_tree <= tree {
                    break;
                }
            }

            let mut visibility_left = 0;
            for &tree in trees[i][0..j].iter().rev() {
                visibility_left += 1;

                if this_tree <= tree {
                    break;
                }
            }

            let mut visibility_bottom = 0;
            for tree_row in &trees[i + 1..height] {
                let tree = tree_row[j];

                visibility_bottom += 1;

                if this_tree <= tree {
                    break;
                }
            }

            let mut visibility_right = 0;
            for &tree in &trees[i][j + 1..width] {
                visibility_right += 1;

                if this_tree <= tree {
                    break;
                }
            }

            visibility_scores
                .push(visibility_top * visibility_left * visibility_bottom * visibility_right);
        }
    }

    let part_2_sum = visibility_scores
        .iter()
        .max()
        .ok_or("could not get max visibility score")?;

    println!("Part 1: {}", part_1_sum);
    println!("Part 1: {}", part_2_sum);

    Ok(())
}
