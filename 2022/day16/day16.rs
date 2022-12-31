use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug)]
struct Valve {
    connections: Vec<String>,
    flow_rate: usize,
}

fn find_shortest_path<'a>(
    valves: &'a HashMap<&'a str, Valve>,
    origin: &'a str,
    destination: &str,
) -> Vec<&'a str> {
    // breadth-first search
    let mut nodes_to_check = VecDeque::new();
    nodes_to_check.push_back((origin, vec![]));
    while nodes_to_check.len() > 0 {
        let (current_node, path_to_current_node) = nodes_to_check.pop_front().unwrap();
        for connection in valves[&current_node].connections.iter() {
            let mut path_to_connection = path_to_current_node.clone();
            path_to_connection.push(connection.as_str());
            if connection == destination {
                return path_to_connection;
            }
            nodes_to_check.push_back((&connection, path_to_connection));
        }
    }
    panic!(
        "Could not find a route between {} and {}",
        origin, destination
    );
}

// Note that this only finds paths between nodes with non-zero flow rate
fn find_shortest_paths<'a>(
    valves: &'a HashMap<&'a str, Valve>,
) -> HashMap<(&'a str, &'a str), Vec<&str>> {
    let mut shortest_paths = HashMap::new();
    for (origin, _valve) in valves
        .iter()
        .filter(|(n, v)| v.flow_rate > 0 || **n == "AA")
    {
        for (destination, _valve) in valves.iter().filter(|(_, v)| v.flow_rate > 0) {
            if origin == destination {
                continue;
            }
            shortest_paths.insert(
                (*origin, *destination),
                find_shortest_path(valves, origin, destination),
            );
        }
    }
    shortest_paths
}

fn parse(data: &str) -> HashMap<&str, Valve> {
    let mut valves = HashMap::new();
    for line in data.split("\n") {
        let valve_name = &line[6..8];
        let (flow_rate, mut rest) = line[23..].split_once(';').unwrap();
        let flow_rate = flow_rate.parse().unwrap();
        // Curse you single path with single tunnel for breaking my parsing
        if rest.chars().nth(7).unwrap() == 's' {
            rest = &rest[24..]; // " tunnels lead to valves "
        } else {
            rest = &rest[23..]; // " tunnel leads to valve "
        }
        let connections = rest.split(", ").map(|s| s.to_owned()).collect();
        valves.insert(
            valve_name,
            Valve {
                connections: connections,
                flow_rate: flow_rate,
            },
        );
    }
    valves
}

fn process(data: &str) -> usize {
    let valves = parse(data);
    let mut starting_closed_flowing_valves: HashSet<&str> = valves
        .iter()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(k, _)| *k)
        .collect();
    let shortest_paths = find_shortest_paths(&valves);

    let mut most_pressure_released = 0;

    // breadth-first search
    let mut nodes_to_check = VecDeque::new(); // (current_node, remaining_closed_valves, pressure_released, minutes_elapsed)
    if starting_closed_flowing_valves.contains("AA") {
        starting_closed_flowing_valves.remove("AA");
        nodes_to_check.push_back((
            "AA",
            starting_closed_flowing_valves,
            29 * valves["AA"].flow_rate,
            1,
        ));
    } else {
        nodes_to_check.push_back(("AA", starting_closed_flowing_valves, 0, 0));
    }
    while let Some((current_node, remaining_closed_valves, pressure_released, minutes_elapsed)) =
        nodes_to_check.pop_front()
    {
        for target_node in remaining_closed_valves.iter() {
            // can we reach it before 30 minutes is up?
            let opened_at =
                minutes_elapsed + shortest_paths[&(current_node, *target_node)].len() + 1;
            if opened_at < 30 {
                let new_pressure_released =
                    pressure_released + (30 - opened_at) * valves[target_node].flow_rate;
                let mut new_remaining_closed_valves = remaining_closed_valves.clone();
                new_remaining_closed_valves.remove(target_node);
                nodes_to_check.push_back((
                    target_node,
                    new_remaining_closed_valves,
                    new_pressure_released,
                    opened_at,
                ));
                if new_pressure_released > most_pressure_released {
                    most_pressure_released = new_pressure_released;
                }
            }
        }
    }
    most_pressure_released
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part1() {
        assert_eq!(process(DATA), 1651);
    }
}
