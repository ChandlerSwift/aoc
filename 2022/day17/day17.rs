use std::collections::VecDeque;
use std::fs;

fn height(chamber: &Vec<[bool; 7]>) -> usize {
    for (i, row) in chamber.iter().enumerate().rev() {
        if row.iter().any(|x| *x) {
            // Is there a better function here?
            return i + 1;
        }
    }

    panic!("No row full");
}

#[derive(Debug)]
enum JetDirection {
    Left,
    Right,
}

fn render(chamber: &Vec<[bool; 7]>, falling_points: &Vec<(i32, i32)>) {
    for (y, row) in chamber.iter().enumerate().rev() {
        print!("|");
        for (x, c) in row.iter().enumerate() {
            if *c {
                print!("#");
            } else if falling_points.contains(&(x as i32, y as i32)) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
    println!();
}

fn process(data: &str) -> usize {
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

    let mut jets = VecDeque::new();
    for c in data.chars() {
        jets.push_back(if c == '<' {
            JetDirection::Left
        } else {
            assert_eq!(c, '>'); // We should never see any other characters here
            JetDirection::Right
        });
    }

    let mut falling_points = Vec::new();
    let mut chamber = vec![[false; 7]; 4];
    let mut current_falling_rock = 0;
    let mut is_moving_sideways = true;
    for point in &rock_shapes[0] {
        falling_points.push((point.0 + 2, point.1 + 3));
    }
    while current_falling_rock < 2022 {
        let target_offset;
        if is_moving_sideways {
            let jet_direction = jets.pop_front().unwrap();
            target_offset = match jet_direction {
                JetDirection::Left => (-1, 0),
                JetDirection::Right => (1, 0),
            };
            jets.push_back(jet_direction);
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
            // Convert all falling points into stuck points
            while let Some((x, y)) = falling_points.pop() {
                chamber[y as usize][x as usize] = true;
            }

            current_falling_rock += 1;

            for point in &rock_shapes[current_falling_rock % 5] {
                let h = height(&chamber);
                for _ in 0..h + 7 - chamber.len() {
                    chamber.push([false; 7]);
                }
                falling_points.push((point.0 + 2, point.1 + h as i32 + 3));
            }
        }
        is_moving_sideways = !is_moving_sideways;
    }

    height(&chamber)
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(process(DATA), 3068);
    }
}
