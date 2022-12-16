use std::{cmp::max, collections::HashMap, error::Error};

use itertools::Itertools;
use utils::read_input_file;

struct Storage<'a> {
    flow_rates: HashMap<&'a str, usize>,
    adjacent_valves: HashMap<&'a str, Vec<&'a str>>,
    lookup: HashMap<(&'a str, Vec<&'a str>, i32), usize>,
}

impl<'a> Storage<'a> {
    fn new(
        flow_rates: HashMap<&'a str, usize>,
        adjacent_valves: HashMap<&'a str, Vec<&'a str>>,
    ) -> Self {
        Self {
            flow_rates,
            adjacent_valves,
            lookup: HashMap::new(),
        }
    }
}

fn max_flow<'a>(
    storage: &'a mut Storage<'a>,
    current_valve: &'a str,
    open_valves: Vec<&'a str>,
    minutes_left: i32,
) -> usize {
    if let Some(&flow_rate) =
        storage
            .lookup
            .get(&(current_valve, open_valves.clone(), minutes_left))
    {
        return flow_rate;
    }

    if minutes_left <= 0 {
        return 0;
    }

    let mut best_flow = 0;

    if !open_valves.contains(&current_valve) {
        let flow = (minutes_left - 1) as usize * storage.flow_rates[current_valve];

        let mut open_valves = open_valves.clone();
        open_valves.push(current_valve);
        open_valves.sort();

        let adjacent_valves = storage.adjacent_valves.clone();
        for valve in &adjacent_valves[current_valve] {
            if flow != 0 {
                best_flow = max(
                    best_flow,
                    flow + max_flow(storage, valve, open_valves.clone(), minutes_left - 2),
                );

                storage
                    .lookup
                    .insert((valve, open_valves.clone(), minutes_left - 2), best_flow);
            }

            best_flow = max(
                best_flow,
                max_flow(storage, valve, open_valves.clone(), minutes_left - 1),
            );

            storage
                .lookup
                .insert((valve, open_valves.clone(), minutes_left - 1), best_flow);
        }
    }

    best_flow
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut flow_rates = HashMap::new();
    let mut adjacent_valves = HashMap::new();
    for line in file_content.lines() {
        let (valve_part, adjacent_valves_part) =
            line.split_once(';').ok_or("could not split line")?;

        let valve = &valve_part[6..8];
        let flow_rate = valve_part[23..].parse::<usize>()?;
        let valves = if adjacent_valves_part.contains("valves") {
            adjacent_valves_part[24..].split(", ").collect_vec()
        } else {
            adjacent_valves_part[23..].split(", ").collect_vec()
        };
        flow_rates.insert(valve, flow_rate);
        adjacent_valves.insert(valve, valves);
    }

    let mut storage = Storage::new(flow_rates, adjacent_valves);

    let part_1 = max_flow(&mut storage, "AA", Vec::new(), 30);

    println!("Part 1: {}", part_1);

    Ok(())
}
