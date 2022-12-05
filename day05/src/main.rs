use std::error::Error;

use itertools::Itertools;
use utils::read_input_file;

type Crate = char;

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<Crate>,
}

impl Stack {
    fn new() -> Self {
        Self { crates: Vec::new() }
    }

    fn add(&mut self, c: Crate) {
        self.crates.push(c)
    }

    fn remove(&mut self) -> Option<Crate> {
        self.crates.pop()
    }

    fn remove_n(&mut self, n: usize) -> Result<Vec<Crate>, Box<dyn Error>> {
        let mut res = Vec::new();
        for _ in 0..n {
            res.push(self.crates.pop().ok_or("coult not remove item from stack")?);
        }
        res.reverse();
        Ok(res)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let (crate_layout, commands) = file_content
        .split_once("\n\n")
        .ok_or("could not split crate layout from the operations")?;

    let mut layout_it = crate_layout.lines().rev();
    let numbers = layout_it.next().ok_or("could not get layout numbers")?;
    let rows: Vec<_> = layout_it.collect();

    let stack_count = numbers.chars().rev().next().unwrap().to_digit(10).unwrap() as usize;
    let mut stacks: Vec<Stack> = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        stacks.push(Stack::new());
    }

    for row in rows {
        for i in 0..stack_count {
            if let Some((_, crate_content, _)) = row.chars().skip(i * 3 + i).take(3).collect_tuple()
            {
                if crate_content != ' ' {
                    let stack = stacks
                        .get_mut(i)
                        .ok_or(format!("could not get stack with index {}", i))?;
                    stack.add(crate_content);
                }
            }
        }
    }

    let mut part_1_stacks = stacks.to_vec();

    for command in commands.lines() {
        let parts = command.split_whitespace().collect::<Vec<_>>();
        let count = parts[1].parse::<u32>()?;
        let from = parts[3].parse::<usize>()? - 1;
        let to = parts[5].parse::<usize>()? - 1;

        let from_stack = part_1_stacks
            .get_mut(from)
            .ok_or(format!("could not get from-stack with index {}", from))?;
        let crates_to_add = (0..count)
            .map(|_| from_stack.remove().unwrap())
            .collect_vec();

        let to_stack = part_1_stacks
            .get_mut(to)
            .ok_or(format!("could not get to-stack with index {}", to))?;
        for crate_to_add in crates_to_add {
            to_stack.add(crate_to_add);
        }
    }

    let mut part_1 = String::new();
    for stack in part_1_stacks.as_mut_slice() {
        if let Some(c) = stack.remove() {
            part_1.push(c);
        }
    }

    let mut part_2_stacks = stacks.to_vec();

    for command in commands.lines() {
        let parts = command.split_whitespace().collect::<Vec<_>>();
        let count = parts[1].parse::<usize>()?;
        let from = parts[3].parse::<usize>()? - 1;
        let to = parts[5].parse::<usize>()? - 1;

        println!("{:?}", part_2_stacks);
        println!("move {} from {} to {}", count, from, to);

        let from_stack = part_2_stacks
            .get_mut(from)
            .ok_or(format!("could not get from-stack with index {}", from))?;
        let crates_to_add = from_stack.remove_n(count)?;

        let to_stack = part_2_stacks
            .get_mut(to)
            .ok_or(format!("could not get to-stack with index {}", to))?;
        for crate_to_add in crates_to_add {
            to_stack.add(crate_to_add);
        }
    }

    let mut part_2 = String::new();
    for stack in part_2_stacks.as_mut_slice() {
        if let Some(c) = stack.remove() {
            part_2.push(c);
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}
