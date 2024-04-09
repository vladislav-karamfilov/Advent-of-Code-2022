use std::collections::{HashMap, HashSet};

fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let network = read_network();

    let mut states = vec![];
    states.push((1, "AA".to_string(), 0 as u32, HashSet::new()));

    let mut cache = HashMap::new();

    let mut max_released_pressure = 0 as u32;

    while !states.is_empty() {
        let (current_time, current_valve_label, current_pressure, mut opened_valves) =
            states.pop().unwrap();

        match cache.get(&(current_time, current_valve_label.clone())) {
            Some(cached_pressure) => {
                if *cached_pressure >= current_pressure {
                    continue;
                }
            }
            None => {}
        }

        cache.insert(
            (current_time, current_valve_label.clone()),
            current_pressure,
        );

        if current_time == 30 {
            max_released_pressure = max_released_pressure.max(current_pressure);
            continue;
        }

        let current_valve = &network[&current_valve_label];

        // Try opening the valve
        if current_valve.flow_rate > 0 && !opened_valves.contains(&current_valve.label) {
            opened_valves.insert(current_valve.label.clone());

            let new_pressure = current_pressure
                + opened_valves
                    .iter()
                    .map(|v| network[v].flow_rate)
                    .sum::<u32>();

            states.push((
                current_time + 1,
                current_valve.label.clone(),
                new_pressure,
                opened_valves.clone(),
            ));

            opened_valves.remove(&current_valve.label);
        }

        // Try moving to connected valves
        let new_pressure = current_pressure
            + opened_valves
                .iter()
                .map(|v| network[v].flow_rate)
                .sum::<u32>();

        for connected_valve in &current_valve.connected_valves {
            states.push((
                current_time + 1,
                connected_valve.clone(),
                new_pressure,
                opened_valves.clone(),
            ));
        }
    }

    println!("{max_released_pressure}");
}

fn read_network() -> HashMap<String, Valve> {
    let mut network = HashMap::new();

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let label_start_index = line.find(' ').unwrap() + 1;
        let label_end_index = line[label_start_index..].find(' ').unwrap() + label_start_index;
        let label = line[label_start_index..label_end_index].to_string();

        let flow_rate_start_index =
            line[label_end_index..].find('=').unwrap() + 1 + label_end_index;
        let flow_rate_end_index = line[label_end_index..].find(';').unwrap() + label_end_index;
        let flow_rate = line[flow_rate_start_index..flow_rate_end_index]
            .parse::<u32>()
            .unwrap();

        let connected_valves_start_index = if line.contains("valves") {
            line.rfind("valves").unwrap() + "valves".len() + 1
        } else {
            line.rfind("valve").unwrap() + "valve".len() + 1
        };

        let connected_valves: Vec<String> = line[connected_valves_start_index..]
            .split(", ")
            .map(|v| v.to_string())
            .collect();

        network.insert(
            label.clone(),
            Valve {
                label,
                flow_rate,
                connected_valves,
            },
        );
    }

    network
}

struct Valve {
    label: String,
    flow_rate: u32,
    connected_valves: Vec<String>,
}
