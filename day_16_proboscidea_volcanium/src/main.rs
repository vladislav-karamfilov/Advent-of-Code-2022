use std::collections::{HashMap, HashSet};

fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let network = read_network();

    let mut opened_valves = HashSet::new();
    let mut released_pressures = HashSet::new();

    let mut cache = HashSet::new();

    open_valves(
        0,
        &network["AA"],
        &network,
        30,
        &mut opened_valves,
        &mut released_pressures,
        &mut cache,
    );

    let most_released_pressure = released_pressures.iter().max().unwrap();

    println!("{most_released_pressure}");
}

fn open_valves(
    current_released_pressure: u32,
    current_valve: &Valve,
    network: &HashMap<String, Valve>,
    remaining_minutes: u32,
    opened_valves: &mut HashSet<String>,
    released_pressures: &mut HashSet<u32>,
    cache: &mut HashSet<String>,
) {
    if remaining_minutes <= 1
        || opened_valves.len() == network.iter().filter(|(_, v)| v.flow_rate > 0).count()
    {
        released_pressures.insert(current_released_pressure);
        return;
    }

    let params = format!(
        "{} | {} | {} | {} | {}",
        current_released_pressure,
        &current_valve.label,
        remaining_minutes,
        &opened_valves
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
            .join(", "),
        &released_pressures
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    if !cache.insert(params.clone()) {
        println!("{params}");
        panic!("asd");
    }

    if !opened_valves.contains(&current_valve.label) && current_valve.flow_rate > 0 {
        let new_released_pressure =
            current_released_pressure + current_valve.flow_rate * (remaining_minutes - 1);
        opened_valves.insert(current_valve.label.clone());

        open_valves(
            new_released_pressure,
            current_valve,
            network,
            remaining_minutes - 1,
            opened_valves,
            released_pressures,
            cache,
        );

        opened_valves.remove(&current_valve.label);
    }

    for connected_valve in &current_valve.connected_valves {
        open_valves(
            current_released_pressure,
            &network[connected_valve],
            network,
            remaining_minutes - 1,
            opened_valves,
            released_pressures,
            cache,
        );
    }
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
