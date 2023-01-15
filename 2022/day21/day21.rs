use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy)]
enum Monkey<'a> {
    Const(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

fn eval_monkey(monkeys: &HashMap<&str, Monkey>, addr: &str) -> i64 {
    let monkey = monkeys[addr];
    match monkey {
        Monkey::Const(a) => a,
        Monkey::Add(a, b) => eval_monkey(&monkeys, a) + eval_monkey(&monkeys, b),
        Monkey::Sub(a, b) => eval_monkey(&monkeys, a) - eval_monkey(&monkeys, b),
        Monkey::Mul(a, b) => eval_monkey(&monkeys, a) * eval_monkey(&monkeys, b),
        Monkey::Div(a, b) => eval_monkey(&monkeys, a) / eval_monkey(&monkeys, b),
    }
}

fn get_value_for(
    monkeys: &HashMap<&str, Monkey>,
    monkeys_used_by: &HashMap<&str, &str>,
    addr: &str,
) -> i64 {
    let parent = monkeys_used_by[addr];
    let (parent_first_child, parent_second_child) = match monkeys[parent] {
        Monkey::Const(_) => panic!("We're a child of const"),
        Monkey::Add(a, b) => (a, b),
        Monkey::Sub(a, b) => (a, b),
        Monkey::Mul(a, b) => (a, b),
        Monkey::Div(a, b) => (a, b),
    };
    if parent == "root" {
        if parent_first_child == addr {
            eval_monkey(&monkeys, parent_second_child)
        } else {
            eval_monkey(&monkeys, parent_first_child)
        }
    } else {
        match monkeys[parent] {
            Monkey::Const(_) => panic!("We're a child of const"),
            Monkey::Add(a, b) => {
                if a == addr {
                    get_value_for(&monkeys, &monkeys_used_by, parent) - eval_monkey(&monkeys, b)
                } else {
                    get_value_for(&monkeys, &monkeys_used_by, parent) - eval_monkey(&monkeys, a)
                }
            }
            Monkey::Sub(a, b) => {
                if a == addr {
                    get_value_for(&monkeys, &monkeys_used_by, parent) + eval_monkey(&monkeys, b)
                } else {
                    eval_monkey(&monkeys, a) - get_value_for(&monkeys, &monkeys_used_by, parent)
                }
            }
            Monkey::Mul(a, b) => {
                if a == addr {
                    get_value_for(&monkeys, &monkeys_used_by, parent) / eval_monkey(&monkeys, b)
                } else {
                    get_value_for(&monkeys, &monkeys_used_by, parent) / eval_monkey(&monkeys, a)
                }
            }
            Monkey::Div(a, b) => {
                if a == addr {
                    get_value_for(&monkeys, &monkeys_used_by, parent) * eval_monkey(&monkeys, b)
                } else {
                    eval_monkey(&monkeys, a) / get_value_for(&monkeys, &monkeys_used_by, parent)
                }
            }
        }
    }
}

fn parse(data: &str) -> (HashMap<&str, Monkey>, HashMap<&str, &str>) {
    let mut monkeys: HashMap<&str, Monkey> = HashMap::new();
    let mut monkeys_used_by: HashMap<&str, &str> = HashMap::new();
    for row in data.split("\n") {
        let (monkey, op) = row.split_once(": ").unwrap();
        let op: Vec<&str> = op.split(" ").collect();
        monkeys.insert(
            monkey,
            match op.len() {
                1 => Monkey::Const(op[0].parse().unwrap()),
                3 => {
                    assert_eq!(monkeys_used_by.insert(op[0], monkey), None);
                    assert_eq!(monkeys_used_by.insert(op[2], monkey), None);
                    match op[1] {
                        "+" => Monkey::Add(op[0], op[2]),
                        "-" => Monkey::Sub(op[0], op[2]),
                        "*" => Monkey::Mul(op[0], op[2]),
                        "/" => Monkey::Div(op[0], op[2]),
                        other => panic!("Unknown operator {}", other),
                    }
                }
                _ => panic!("Unknown operator {:?}", op),
            },
        );
    }

    (monkeys, monkeys_used_by)
}

fn process1(data: &str) -> i64 {
    let (monkeys, _monkeys_used_by) = parse(data);
    eval_monkey(&monkeys, "root")
}

fn process2(data: &str) -> i64 {
    let (monkeys, monkeys_used_by) = parse(data);

    get_value_for(&monkeys, &monkeys_used_by, "humn")
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process2(data));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part1() {
        assert_eq!(process1(DATA), 152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process2(DATA), 301);
    }
}
