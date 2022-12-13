use std::{collections::VecDeque, error::Error, fmt::Display, str::FromStr};

use itertools::Itertools;
use utils::read_input_file;

struct Monkey {
    items: VecDeque<usize>,
    items_visited: usize,
    operation: Box<dyn Fn(usize) -> usize>,
    test_divisor: usize,
    true_throw_to: usize,
    false_throw_to: usize,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "items = {:?}", self.items)?;
        writeln!(f, "items_visited = {}", self.items_visited)?;
        writeln!(f, "test_divisor = {}", self.test_divisor)?;
        writeln!(f, "true_throw_to = {}", self.true_throw_to)?;
        writeln!(f, "false_throw_to = {}", self.false_throw_to)?;
        Ok(())
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();

        let starting_items = lines[1]
            .strip_prefix("  Starting items: ")
            .ok_or("could not extract starting items")?;
        let starting_items = starting_items
            .split(", ")
            .map(|item| item.parse::<usize>())
            .fold_ok(VecDeque::new(), |mut acc, item| {
                acc.push_back(item);
                acc
            })?;

        let operation = lines[2]
            .strip_prefix("  Operation: new = ")
            .ok_or("could not extract operation")?;
        let operation = operation.split_whitespace().collect_vec();
        let operation: Box<dyn Fn(usize) -> usize> = match operation.as_slice() {
            ["old", "*", "old"] => Box::new(|old: usize| old * old),
            ["old", "*", count] => {
                let count = count.parse::<usize>()?;
                Box::new(move |old: usize| old * count)
            }
            ["old", "+", count] => {
                let count = count.parse::<usize>()?;
                Box::new(move |old: usize| old + count)
            }
            _ => return Err("invalid operation".into()),
        };

        let test_divisor = lines[3]
            .strip_prefix("  Test: divisible by ")
            .ok_or("could not extract test divisor")?;
        let test_divisor = test_divisor.parse::<usize>()?;

        let true_throw_to = lines[4]
            .strip_prefix("    If true: throw to monkey ")
            .ok_or("could not extract monkey to throw to if true")?;
        let true_throw_to = true_throw_to.parse::<usize>()?;

        let false_throw_to = lines[5]
            .strip_prefix("    If false: throw to monkey ")
            .ok_or("could not extract monkey to throw to if false")?;
        let false_throw_to = false_throw_to.parse::<usize>()?;

        Ok(Self {
            items: starting_items,
            items_visited: 0,
            operation,
            test_divisor,
            true_throw_to,
            false_throw_to,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut monkeys = Vec::new();
    for monkey in file_content.split("\n\n") {
        monkeys.push(monkey.parse::<Monkey>()?);
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).ok_or("could not get monkey")?;

            let mut items_to_be_moved = Vec::with_capacity(monkey.items.len());
            for item in monkey.items.drain(..) {
                let new_item = (*monkey.operation)(item) / 3;

                if new_item % monkey.test_divisor == 0 {
                    items_to_be_moved.push((monkey.true_throw_to, new_item));
                } else {
                    items_to_be_moved.push((monkey.false_throw_to, new_item));
                }

                monkey.items_visited += 1;
            }

            for (i, item) in items_to_be_moved {
                monkeys[i].items.push_back(item);
            }
        }
    }

    let items_visited_per_monkey = monkeys
        .iter()
        .map(|monkey| monkey.items_visited)
        .sorted_by(|a, b| b.cmp(a))
        .collect_vec();

    let part_1_sum = items_visited_per_monkey.iter().take(2).product::<usize>();

    let mut monkeys = Vec::new();
    for monkey in file_content.split("\n\n") {
        monkeys.push(monkey.parse::<Monkey>()?);
    }

    let modulo = monkeys
        .iter()
        .map(|monkey| monkey.test_divisor)
        .product::<usize>();

    for _ in 0..10000 {
        let monkey_count = monkeys.len();
        for i in 0..monkey_count {
            let monkey = monkeys.get_mut(i).ok_or("could not get monkey")?;

            let mut items_to_be_moved = Vec::with_capacity(monkey.items.len());
            for item in monkey.items.drain(..) {
                let new_item = (*monkey.operation)(item) % modulo;

                if new_item % monkey.test_divisor == 0 {
                    items_to_be_moved.push((monkey.true_throw_to, new_item));
                } else {
                    items_to_be_moved.push((monkey.false_throw_to, new_item));
                }

                monkey.items_visited += 1;
            }

            for (i, item) in items_to_be_moved {
                monkeys[i].items.push_back(item);
            }
        }
    }

    let items_visited_per_monkey = monkeys
        .iter()
        .map(|monkey| monkey.items_visited)
        .sorted_by(|a, b| b.cmp(a))
        .collect_vec();

    let part_2_sum = items_visited_per_monkey.iter().take(2).product::<usize>();

    println!("Part 1: {}", part_1_sum);
    println!("Part 2: {}", part_2_sum);

    Ok(())
}
