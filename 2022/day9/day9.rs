use std::collections::HashMap;
use std::fs;

fn do_move(
    direction: &str,
    head_pos: (i32, i32),
    tail_pos: (i32, i32),
) -> ((i32, i32), (i32, i32)) {
    let mut head_pos = head_pos;
    let mut tail_pos = tail_pos;

    let old_head_pos = head_pos.clone();

    match direction {
        "U" => head_pos.1 += 1,
        "D" => head_pos.1 -= 1,
        "L" => head_pos.0 -= 1,
        "R" => head_pos.0 += 1,
        _ => panic!("Illegal direction {}", direction),
    }

    if (head_pos.0 - tail_pos.0).abs() > 1 || (head_pos.1 - tail_pos.1).abs() > 1 {
        tail_pos = old_head_pos;
    }

    (head_pos, tail_pos)
}

fn process(data: &str) -> usize {
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut visited_positions = HashMap::new();
    for row in data.split("\n") {
        let command: Vec<&str> = row.split(" ").collect();
        let direction = command[0];
        for _ in 0..command[1].parse().unwrap() {
            (head_pos, tail_pos) = do_move(direction, head_pos, tail_pos);
            visited_positions.insert(tail_pos, true);
        }
    }
    visited_positions.len()
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data));
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
            // head, tail, dir, new_head, new_tail
            ((1, 0), (0, 0), "R", (2, 0), (1, 0)),
            ((1, -2), (1, -1), "D", (1, -3), (1, -2)),
            ((2, -2), (1, -3), "R", (3, -2), (2, -2)),
        ];
        for (head, tail, dir, new_head, new_tail) in cases {
            println!(
                "{:?} == {:?}",
                do_move(dir, head, tail),
                (new_head, new_tail)
            );
            assert!(do_move(dir, head, tail) == (new_head, new_tail));
        }
    }

    #[test]
    fn test_full() {
        assert!(process(DATA) == 13);
    }
}
