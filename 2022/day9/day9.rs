use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn do_move(direction: &str, positions: &mut Vec<(i32, i32)>) {
    match direction {
        "U" => positions[0].1 += 1,
        "D" => positions[0].1 -= 1,
        "L" => positions[0].0 -= 1,
        "R" => positions[0].0 += 1,
        _ => panic!("Illegal direction {}", direction),
    }

    for i in 1..positions.len() {
        if (positions[i].0 - positions[i - 1].0).abs() > 1
            || (positions[i].1 - positions[i - 1].1).abs() > 1
        {
            positions[i].0 += match positions[i].0.cmp(&positions[i - 1].0) {
                Ordering::Less => 1, // TODO: There _must_ be a better way to do this
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };
            positions[i].1 += match positions[i].1.cmp(&positions[i - 1].1) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };
        }
    }
}

fn process(data: &str, knot_count: usize) -> usize {
    let mut positions = Vec::new();
    for _ in 0..knot_count {
        positions.push((0, 0));
    }
    let mut visited_positions = HashMap::new();
    for row in data.split("\n") {
        let command: Vec<&str> = row.split(" ").collect();
        let direction = command[0];
        for _ in 0..command[1].parse().unwrap() {
            do_move(direction, &mut positions);
            visited_positions.insert(positions[positions.len() - 1], true);
        }
    }
    visited_positions.len()
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data, 10));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_move() {
        let cases = vec![
            // positions, dir, new_positions
            (vec![(1, 0), (0, 0)], "R", vec![(2, 0), (1, 0)]),
            (vec![(1, -2), (1, -1)], "D", vec![(1, -3), (1, -2)]),
            (vec![(2, -2), (1, -3)], "R", vec![(3, -2), (2, -2)]),
        ];
        for (original_positions, dir, new_positions) in cases {
            let mut positions = original_positions.clone();
            do_move(dir, &mut positions);
            println!("{:?} == {:?}", positions, original_positions);
            assert!(positions == new_positions);
        }
    }

    #[test]
    fn test_full() {
        assert!(process(DATA, 2) == 13);
        assert!(process(DATA, 10) == 1);
    }

    #[test]
    fn test_fuller() {
        let data = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert!(process(data, 10) == 36);
    }
}
