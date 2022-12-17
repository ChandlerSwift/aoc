use std::collections::VecDeque;
use std::fs;

#[derive(Eq, PartialEq, Debug)]
struct Monkey {
    starting_items: VecDeque<usize>,
    operation: Vec<String>,
    test_divisible_by_cond: usize,
    target_if_test: usize,
    target_if_not_test: usize,
    inspected_item_count: usize,
}

fn update_worry_level(operation: &Vec<String>, level: usize) -> usize {
    if operation[4] == "old" {
        // new = old * old
        level * level
    } else {
        match operation[3].as_str() {
            "*" => level * operation[4].parse::<usize>().unwrap(),
            "+" => level + operation[4].parse::<usize>().unwrap(),
            _ => panic!("Unknown operator {}", operation[3]),
        }
    }
}

fn parse(data: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for monkey_data in data.split("\n\n") {
        let lines: Vec<&str> = monkey_data.split("\n").collect();
        monkeys.push(Monkey {
            starting_items: lines[1][18..]
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect(),
            operation: lines[2][13..].split(" ").map(|s| s.to_string()).collect(),
            test_divisible_by_cond: lines[3][21..].parse().unwrap(),
            target_if_test: lines[4][29..].parse().unwrap(),
            target_if_not_test: lines[5][30..].parse().unwrap(),
            inspected_item_count: 0,
        });
    }
    monkeys
}

fn process(monkeys: &mut Vec<Monkey>, rounds: usize) -> usize {
    let test_product: usize = monkeys.iter().map(|m| m.test_divisible_by_cond).product();
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            while monkeys[i].starting_items.len() > 0 {
                monkeys[i].inspected_item_count += 1;
                let mut item = monkeys[i].starting_items.pop_front().unwrap();
                item = update_worry_level(&monkeys[i].operation, item);
                // item /= 3; // part 1 only
                item %= test_product; // part 2 only
                let target_monkey = if item % monkeys[i].test_divisible_by_cond == 0 {
                    monkeys[i].target_if_test
                } else {
                    monkeys[i].target_if_not_test
                };
                monkeys[target_monkey].starting_items.push_back(item);
            }
        }
    }
    let mut access_counts: Vec<usize> = monkeys.iter().map(|m| m.inspected_item_count).collect();
    access_counts.sort_unstable_by(|a, b| b.cmp(a));
    access_counts[0] * access_counts[1]
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    let mut monkeys = parse(data);
    println!("{}", process(&mut monkeys, 10000)); // 20 for part 1
}

#[cfg(test)]
mod test {
    use super::*;

    static DATA: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parse_one_monkey() {
        let data = "Monkey 0:
  Starting items: 75, 63
  Operation: new = old * 3
  Test: divisible by 11
    If true: throw to monkey 7
    If false: throw to monkey 2";
        let monkeys = parse(data);
        println!("{:?}", monkeys);
        let goal_monkeys = vec![Monkey {
            starting_items: VecDeque::from([75, 63]),
            operation: vec!["new", "=", "old", "*", "3"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            test_divisible_by_cond: 11,
            target_if_test: 7,
            target_if_not_test: 2,
            inspected_item_count: 0,
        }];
        println!("Goal:     {:?}\nActual: {:?}", goal_monkeys, monkeys);
        assert!(goal_monkeys == monkeys);
    }
    #[test]
    fn test() {
        let desired_results = vec![
            (1, 4 * 6),
            (20, 99 * 103),
            (1000, 5204 * 5192),
            (2000, 10419 * 10391),
            (3000, 15638 * 15593),
            (4000, 20858 * 20797),
            (5000, 26075 * 26000),
            (6000, 31294 * 31204),
            (7000, 36508 * 36400),
            (8000, 41728 * 41606),
            (9000, 46945 * 46807),
            (10000, 52166 * 52013),
        ];
        for (rounds, desired_result) in desired_results {
            let mut monkeys = parse(DATA);
            let result = process(&mut monkeys, rounds);
            println!("{} == {}", result, desired_result);
            assert!(result == desired_result);
        }
    }
}
