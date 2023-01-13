use std::fs;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Blueprint {
    id: u8,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

#[derive(Debug, Copy, Clone)]
struct Inventory {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

#[derive(Debug, Copy, Clone)]
struct Robots {
    ore_collecting: u32,
    clay_collecting: u32,
    obsidian_collecting: u32,
    geode_cracking: u32,
}

fn parse_blueprint(data: &str) -> Blueprint {
    let (id, remaining) = data[10..].split_once(": ").unwrap();
    let id = id.parse().unwrap();
    let (ore_robot_ore_cost, remaining) = remaining[21..].split_once(" ").unwrap();
    let ore_robot_ore_cost = ore_robot_ore_cost.parse().unwrap();
    let (clay_robot_ore_cost, remaining) = remaining[27..].split_once(" ").unwrap();
    let clay_robot_ore_cost = clay_robot_ore_cost.parse().unwrap();
    let (obsidian_robot_ore_cost, remaining) = remaining[31..].split_once(" ").unwrap();
    let obsidian_robot_ore_cost = obsidian_robot_ore_cost.parse().unwrap();
    let (obsidian_robot_clay_cost, remaining) = remaining[8..].split_once(" ").unwrap();
    let obsidian_robot_clay_cost = obsidian_robot_clay_cost.parse().unwrap();
    let (geode_robot_ore_cost, remaining) = remaining[29..].split_once(" ").unwrap();
    let geode_robot_ore_cost = geode_robot_ore_cost.parse().unwrap();
    let (geode_robot_obsidian_cost, _remaining) = remaining[8..].split_once(" ").unwrap();
    let geode_robot_obsidian_cost = geode_robot_obsidian_cost.parse().unwrap();
    Blueprint {
        id: id,
        ore_robot_ore_cost: ore_robot_ore_cost,
        clay_robot_ore_cost: clay_robot_ore_cost,
        obsidian_robot_ore_cost: obsidian_robot_ore_cost,
        obsidian_robot_clay_cost: obsidian_robot_clay_cost,
        geode_robot_ore_cost: geode_robot_ore_cost,
        geode_robot_obsidian_cost: geode_robot_obsidian_cost,
    }
}

fn geodes_opened(blueprint: Blueprint, remaining_time: usize) -> usize {
    let robots = Robots {
        ore_collecting: 1,
        clay_collecting: 0,
        obsidian_collecting: 0,
        geode_cracking: 0,
    };
    let inventory = Inventory {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
    };
    rec_geodes_opened(blueprint, remaining_time, robots, inventory, &mut 0)
}

fn rec_geodes_opened(
    blueprint: Blueprint,
    remaining_time: usize,
    robots: Robots,
    inventory: Inventory,
    max_overall_geodes: &mut usize,
) -> usize {
    if remaining_time > 26 {
        println!("{}", remaining_time);
    }
    let starting_inventory = inventory;

    let mut inventory = inventory;
    inventory.ore += robots.ore_collecting as u32;
    inventory.clay += robots.clay_collecting as u32;
    inventory.obsidian += robots.obsidian_collecting as u32;
    inventory.geodes += robots.geode_cracking as u32;

    let possible_geodes: usize = inventory.geodes as usize
        + (robots.geode_cracking as usize * remaining_time)
        + (remaining_time * (remaining_time - 1) / 2);

    let mut max_geodes = 0;

    let mut ore_costs = [
        blueprint.ore_robot_ore_cost,
        blueprint.clay_robot_ore_cost,
        blueprint.obsidian_robot_ore_cost,
        blueprint.geode_robot_ore_cost,
    ];
    ore_costs.sort();
    let max_ore_cost = ore_costs[ore_costs.len() - 1];

    if remaining_time > 1 && possible_geodes >= *max_overall_geodes {
        if blueprint.geode_robot_ore_cost <= starting_inventory.ore
            && blueprint.geode_robot_obsidian_cost <= starting_inventory.obsidian
        {
            let mut robots = robots.clone();
            let mut inventory = inventory.clone();
            inventory.ore -= blueprint.geode_robot_ore_cost;
            inventory.obsidian -= blueprint.geode_robot_obsidian_cost;
            robots.geode_cracking += 1;
            let geodes = rec_geodes_opened(
                blueprint,
                remaining_time - 1,
                robots,
                inventory,
                max_overall_geodes,
            );
            if geodes > max_geodes {
                max_geodes = geodes;
            }
        }
        if blueprint.obsidian_robot_ore_cost <= starting_inventory.ore
            && blueprint.obsidian_robot_clay_cost <= starting_inventory.clay
            && robots.obsidian_collecting < blueprint.geode_robot_obsidian_cost
        {
            let mut robots = robots.clone();
            let mut inventory = inventory.clone();
            inventory.ore -= blueprint.obsidian_robot_ore_cost;
            inventory.clay -= blueprint.obsidian_robot_clay_cost;
            robots.obsidian_collecting += 1;
            let geodes = rec_geodes_opened(
                blueprint,
                remaining_time - 1,
                robots,
                inventory,
                max_overall_geodes,
            );
            if geodes > max_geodes {
                max_geodes = geodes;
            }
        }
        if blueprint.clay_robot_ore_cost <= starting_inventory.ore
            && robots.clay_collecting < blueprint.obsidian_robot_clay_cost
        {
            let mut robots = robots.clone();
            let mut inventory = inventory.clone();
            inventory.ore -= blueprint.clay_robot_ore_cost;
            robots.clay_collecting += 1;
            let geodes = rec_geodes_opened(
                blueprint,
                remaining_time - 1,
                robots,
                inventory,
                max_overall_geodes,
            );
            if geodes > max_geodes {
                max_geodes = geodes;
            }
        }
        if blueprint.ore_robot_ore_cost <= starting_inventory.ore
            && robots.ore_collecting < max_ore_cost
        {
            let mut robots = robots.clone();
            let mut inventory = inventory.clone();
            inventory.ore -= blueprint.ore_robot_ore_cost;
            robots.ore_collecting += 1;
            let geodes = rec_geodes_opened(
                blueprint,
                remaining_time - 1,
                robots,
                inventory,
                max_overall_geodes,
            );
            if geodes > max_geodes {
                max_geodes = geodes;
            }
        }
        let geodes = rec_geodes_opened(
            blueprint,
            remaining_time - 1,
            robots,
            inventory,
            max_overall_geodes,
        );
        if geodes > max_geodes {
            max_geodes = geodes;
        }
    } else {
        return inventory.geodes as usize;
    }

    if max_geodes > *max_overall_geodes {
        *max_overall_geodes = max_geodes;
    }
    max_geodes
}

fn process(data: &str) -> usize {
    let blueprints: Vec<Blueprint> = data.split("\n").map(|s| parse_blueprint(s)).collect();

    blueprints
        .iter()
        .map(|b| b.id as usize * geodes_opened(*b, 24) as usize)
        .sum()
}

fn process2(data: &str) -> usize {
    let blueprints: Vec<Blueprint> = data
        .split("\n")
        .take(3)
        .map(|s| parse_blueprint(s))
        .collect();

    blueprints
        .iter()
        .map(|b| geodes_opened(*b, 32) as usize)
        .product()
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process2(data)); // process(data) for part 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_parse() {
        let mut lines = DATA.split("\n");
        let line_1 = Blueprint {
            id: 1,
            ore_robot_ore_cost: 4,
            clay_robot_ore_cost: 2,
            obsidian_robot_ore_cost: 3,
            obsidian_robot_clay_cost: 14,
            geode_robot_ore_cost: 2,
            geode_robot_obsidian_cost: 7,
        };
        let line_2 = Blueprint {
            id: 2,
            ore_robot_ore_cost: 2,
            clay_robot_ore_cost: 3,
            obsidian_robot_ore_cost: 3,
            obsidian_robot_clay_cost: 8,
            geode_robot_ore_cost: 3,
            geode_robot_obsidian_cost: 12,
        };
        assert_eq!(parse_blueprint(lines.next().unwrap()), line_1);
        assert_eq!(parse_blueprint(lines.next().unwrap()), line_2);
    }

    #[test]
    fn test_find_largest_number_of_geodes_that_can_be_opened() {
        let mut lines = DATA.split("\n");
        assert_eq!(geodes_opened(parse_blueprint(lines.next().unwrap()), 24), 9);
        assert_eq!(
            geodes_opened(parse_blueprint(lines.next().unwrap()), 24),
            12
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(process(DATA), 33);
    }
}
