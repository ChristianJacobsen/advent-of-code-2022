use std::error::Error;

use itertools::Itertools;
use utils::{get_two_mut, read_input_file};

type Crate = char;

#[derive(Clone)]
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

    fn move_n_to(
        &mut self,
        n: usize,
        other: &mut Stack,
        batch_move: bool,
    ) -> Result<(), Box<dyn Error>> {
        let mut to_be_moved = Vec::new();

        for _ in 0..n {
            let c = self.crates.pop().ok_or("could not pop from crates")?;
            to_be_moved.push(c);
        }

        if batch_move {
            to_be_moved.reverse();
        }

        for c in to_be_moved {
            other.add(c);
        }

        Ok(())
    }
}

struct StackList {
    stacks: Vec<Stack>,
}

impl StackList {
    fn new(layout: &str) -> Self {
        let reverse_layout = layout.lines().rev().collect_vec();

        let numbers = reverse_layout[0];
        let count = numbers
            .chars()
            .next_back()
            .expect("should have a last element")
            .to_digit(10)
            .expect("should be a digit") as usize;

        let mut stacks: Vec<Stack> = vec![Stack::new(); count];

        let rows = &reverse_layout[1..reverse_layout.len()];
        for row in rows {
            for i in 0..count {
                if let Some((_, content, _)) = row.chars().skip(i * 3 + i).take(3).collect_tuple() {
                    if content != ' ' {
                        let stack = stacks.get_mut(i).expect("there should be a stack");
                        stack.add(content);
                    }
                }
            }
        }

        Self { stacks }
    }

    fn process_command(&mut self, command: &str, batch_move: bool) -> Result<(), Box<dyn Error>> {
        let (_, count, _, from, _, to) = command
            .split_whitespace()
            .collect_tuple()
            .ok_or("could not collect tuple for command")?;

        let count = count.parse::<usize>()?;
        let from = from.parse::<usize>()? - 1;
        let to = to.parse::<usize>()? - 1;

        let (from_stack, to_stack) =
            get_two_mut(&mut self.stacks, from, to).ok_or("could not get from and to stacks")?;

        from_stack.move_n_to(count, to_stack, batch_move)?;

        Ok(())
    }

    fn get_stack_tops(&self) -> String {
        let mut stack_tops = String::new();
        for stack in &self.stacks {
            if let Some(c) = stack.crates.last() {
                stack_tops.push(*c);
            }
        }
        stack_tops
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let (crates_layout, commands) = file_content
        .split_once("\n\n")
        .ok_or("could not split crate layout from the operations")?;

    let mut part_1_stack_list = StackList::new(crates_layout);
    for command in commands.lines() {
        part_1_stack_list.process_command(command, false)?;
    }
    let part_1_stack_tops = part_1_stack_list.get_stack_tops();

    let mut part_2_stack_list = StackList::new(crates_layout);
    for command in commands.lines() {
        part_2_stack_list.process_command(command, true)?;
    }
    let part_2_stack_tops = part_2_stack_list.get_stack_tops();

    println!("Part 1: {}", part_1_stack_tops);
    println!("Part 2: {}", part_2_stack_tops);

    Ok(())
}
