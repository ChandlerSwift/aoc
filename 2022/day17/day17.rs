use std::collections::HashMap;
use std::fs;

fn height(chamber: &Vec<[bool; 7]>) -> usize {
    for (i, row) in chamber.iter().enumerate().rev() {
        if row.iter().any(|x| *x) {
            // Is there a better function here?
            return i + 1;
        }
    }

    0
}

#[derive(Debug)]
enum JetDirection {
    Left,
    Right,
}

fn process(data: &str, rocks_to_fall: u64) -> u64 {
    // I'd have loved to put rock_shapes as a global constant, but
    // unfortunately since rocks are different sizes, rocks have to be
    // `Vec`s not arrays, and that can't be done statically,
    // apparently. Maybe there's a good workaround for this, but I
    // haven't found it. So, we just put it here instead!
    let rock_shapes = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let jets: Vec<JetDirection> = data
        .chars()
        .map(|c| match c {
            '<' => JetDirection::Left,
            '>' => JetDirection::Right,
            _ => panic!("Unknown characters"),
        })
        .collect();

    let mut falling_points = Vec::new();
    let mut chamber = vec![[false; 7]; 4];
    let mut current_falling_rock = 0;
    let mut is_moving_sideways = true;
    for point in &rock_shapes[0] {
        falling_points.push((point.0 + 2, point.1 + 3));
    }
    let mut seen_states: HashMap<([[bool; 7]; 10], usize), (u64, u64)> = HashMap::new();
    let mut jet_index = 0;
    let mut height_offset: u64 = 0;
    while current_falling_rock < rocks_to_fall {
        let target_offset;
        if is_moving_sideways {
            let jet_direction = &jets[jet_index];
            target_offset = match jet_direction {
                JetDirection::Left => (-1, 0),
                JetDirection::Right => (1, 0),
            };
            jet_index = (jet_index + 1) % jets.len();
        } else {
            target_offset = (0, -1);
        }
        let can_move = falling_points.iter().all(|(x, y)| {
            let new_x = x + target_offset.0;
            let new_y = y + target_offset.1;
            (0..7).contains(&new_x) && new_y >= 0 && !chamber[new_y as usize][new_x as usize]
        });
        if can_move {
            for point in falling_points.iter_mut() {
                *point = (point.0 + target_offset.0, point.1 + target_offset.1);
            }
        } else if !is_moving_sideways {
            if height_offset == 0 && height(&chamber) > 10 {
                let mut foo = [[false; 7]; 10];
                foo.clone_from_slice(&chamber[chamber.len() - 10..chamber.len()]);
                let state: ([[bool; 7]; 10], usize) = (foo, jet_index);
                if seen_states.contains_key(&state) {
                    let remaining_falling_rocks = rocks_to_fall - current_falling_rock;
                    let rocks_fall_per_cycle = current_falling_rock - seen_states[&state].0;
                    let height_added_per_cycle = height(&chamber) as u64 - seen_states[&state].1;
                    let cycles = remaining_falling_rocks / rocks_fall_per_cycle;
                    height_offset = cycles * height_added_per_cycle;
                    current_falling_rock += rocks_fall_per_cycle * cycles;
                } else {
                    seen_states.insert(state, (current_falling_rock, height(&chamber) as u64));
                }
            }
            // Convert all falling points into stuck points
            while let Some((x, y)) = falling_points.pop() {
                chamber[y as usize][x as usize] = true;
            }

            current_falling_rock += 1;

            for point in &rock_shapes[current_falling_rock as usize % 5] {
                let h = height(&chamber) as usize;
                for _ in 0..h + 7 - chamber.len() {
                    chamber.push([false; 7]);
                }
                falling_points.push((point.0 + 2, point.1 + h as i64 + 3));
            }
        }
        is_moving_sideways = !is_moving_sideways;
    }

    height(&chamber) as u64 + height_offset
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data, 1_000_000_000_000)); // 2022 for part 1
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(process(DATA, 2022), 3068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process(DATA, 1_000_000_000_000), 1514285714288);
    }
}
