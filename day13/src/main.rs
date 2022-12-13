use std::{
    cell::RefCell,
    cmp::{max, Ordering},
    error::Error,
    str::FromStr,
};

use itertools::Itertools;
use utils::read_input_file;

#[derive(Clone, Debug)]
enum Node<'a> {
    Integer(i32),
    List(Vec<RefCell<Node<'a>>>),
}

impl<'a> FromStr for Node<'a> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut root = None;

        let mut stacks = Vec::new();
        let mut number = Vec::new();

        for c in s.chars() {
            match c {
                '[' => stacks.push(Vec::new()),
                ']' => {
                    if !number.is_empty() {
                        let num: String = number.drain(..).collect();
                        let num = num.parse::<i32>()?;
                        stacks
                            .last_mut()
                            .ok_or("could not get last stack")?
                            .push(RefCell::new(Node::Integer(num)));
                    }

                    let completed = stacks.pop().ok_or("could not pop completed stack")?;
                    if let Some(back) = stacks.last_mut() {
                        back.push(RefCell::new(Node::List(completed)));
                    } else {
                        root = Some(Node::List(completed));
                    }
                }
                ',' => {
                    if !number.is_empty() {
                        let num: String = number.drain(..).collect();
                        let num = num.parse::<i32>()?;
                        stacks
                            .last_mut()
                            .ok_or("could not get last stack")?
                            .push(RefCell::new(Node::Integer(num)));
                    }
                }
                num => number.push(num),
            }
        }

        Ok(root.ok_or("could not parse to root")?)
    }
}

fn cmp_node(first: &Node, second: &Node) -> Option<Ordering> {
    if let Node::List(first_nodes) = first {
        if let Node::List(second_nodes) = second {
            if second_nodes.is_empty() {
                if first_nodes.is_empty() {
                    return Some(Ordering::Equal);
                }

                return Some(Ordering::Greater);
            }

            if first_nodes.is_empty() {
                return Some(Ordering::Less);
            }

            let mut ordering = Ordering::Equal;

            for i in 0..max(first_nodes.len(), second_nodes.len()) {
                let first = first_nodes.get(i);
                let second = second_nodes.get(i);

                if second.is_none() {
                    return Some(Ordering::Greater);
                }

                if first.is_none() {
                    return Some(Ordering::Less);
                }

                let first = first.unwrap().borrow();
                let second = second.unwrap().borrow();

                ordering = match (&*first, &*second) {
                    (Node::Integer(first), Node::Integer(second)) => {
                        if first < second {
                            Ordering::Less
                        } else if first == second {
                            continue;
                        } else {
                            Ordering::Greater
                        }
                    }
                    (Node::Integer(first), second) => {
                        let first = Node::List(vec![RefCell::new(Node::Integer(*first))]);
                        cmp_node(&first, second).unwrap()
                    }
                    (first, Node::Integer(second)) => {
                        let second = Node::List(vec![RefCell::new(Node::Integer(*second))]);
                        cmp_node(first, &second).unwrap()
                    }
                    (first, second) => cmp_node(first, second).unwrap(),
                };

                if ordering != Ordering::Equal {
                    break;
                }
            }

            return Some(ordering);
        }
    }

    None
}


fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut pairs = file_content
        .split("\n\n")
        .map(|s| -> Result<_, Box<dyn Error>> {
            let (first, second) = s.split_once('\n').ok_or("could not split pair")?;
            let first = first.parse::<Node>()?;
            let second = second.parse::<Node>()?;
            Ok((first, second))
        })
        .fold_ok(Vec::new(), |mut acc, pair| {
            acc.push(pair);
            acc
        })?;

    let mut in_order_indices = Vec::new();
    for (i, (first, second)) in pairs.iter().enumerate() {
        if let Some(ordering) = cmp_node(first, second) {
            if ordering == Ordering::Less {
                in_order_indices.push(i + 1);
            }
        }
    }

    let part_1_sum = in_order_indices.iter().sum::<usize>();

    let mut flattened_pairs = pairs
        .drain(..)
        .fold(Vec::new(), |mut acc, (first, second)| {
            acc.push(first);
            acc.push(second);
            acc
        });

    let divider_packet_1 = Node::List(vec![RefCell::new(Node::List(vec![RefCell::new(
        Node::Integer(2),
    )]))]);

    let divider_packet_2 = Node::List(vec![RefCell::new(Node::List(vec![RefCell::new(
        Node::Integer(6),
    )]))]);

    flattened_pairs.push(divider_packet_1.clone());
    flattened_pairs.push(divider_packet_2.clone());

    flattened_pairs.sort_by(|first, second| cmp_node(first, second).unwrap());

    let (divider_packet_1_index, _) = flattened_pairs
        .iter()
        .find_position(|&node| {
            if let Some(ordering) = cmp_node(node, &divider_packet_1) {
                return ordering == Ordering::Equal;
            }
            false
        })
        .ok_or("could not get position of divider_packet_1")?;

    let (divider_packet_2_index, _) = flattened_pairs
        .iter()
        .find_position(|&node| {
            if let Some(ordering) = cmp_node(node, &divider_packet_2) {
                return ordering == Ordering::Equal;
            }
            false
        })
        .ok_or("could not get position of divider_packet_2")?;

    let part_2_sum = (divider_packet_1_index + 1) * (divider_packet_2_index + 1);

    println!("Part 1: {}", part_1_sum);
    println!("Part 2: {}", part_2_sum);

    Ok(())
}
