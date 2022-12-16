use std::{cmp::max, collections::BTreeMap, error::Error};

use cached::proc_macro::cached;
use itertools::Itertools;
use utils::read_input_file;

#[cached(
    key = "String",
    convert = r#"{ format!("{}:{:?}:{}:{}", current_valve, open_valves, minutes_left, part_2) }"#
)]
fn max_flow(
    flow_rates: &BTreeMap<&str, usize>,
    adjacent_valves: &BTreeMap<&str, Vec<&str>>,
    current_valve: &str,
    open_valves: Vec<&str>,
    minutes_left: i32,
    part_2: bool,
) -> usize {
    if minutes_left <= 0 {
        if part_2 {
            return max_flow(flow_rates, adjacent_valves, "AA", open_valves, 26, false);
        } else {
            return 0;
        }
    }

    let mut best_flow = 0;

    if open_valves.binary_search(&current_valve).is_err() {
        let total_flow = (minutes_left - 1) as usize * flow_rates[current_valve];

        let mut currently_open_valves = open_valves.clone();
        currently_open_valves.push(current_valve);
        currently_open_valves.sort();

        if total_flow != 0 {
            best_flow = max(
                best_flow,
                total_flow
                    + max_flow(
                        flow_rates,
                        adjacent_valves,
                        current_valve,
                        currently_open_valves,
                        minutes_left - 1,
                        part_2,
                    ),
            );
        }
    }

    for &valve in &adjacent_valves[current_valve] {
        best_flow = max(
            best_flow,
            max_flow(
                flow_rates,
                adjacent_valves,
                valve,
                open_valves.clone(),
                minutes_left - 1,
                part_2,
            ),
        );
    }

    best_flow
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut flow_rates = BTreeMap::new();
    let mut adjacent_valves = BTreeMap::new();
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

    let part_1 = max_flow(&flow_rates, &adjacent_valves, "AA", Vec::new(), 30, false);
    let part_2 = max_flow(&flow_rates, &adjacent_valves, "AA", Vec::new(), 26, true);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}
