use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn parse_elf_map(data: &str) -> HashSet<(i32, i32)> {
    let mut elf_locations = HashSet::new();
    for (j, row) in data.split("\n").enumerate() {
        for (i, c) in row.chars().enumerate() {
            if c == '#' {
                elf_locations.insert((i as i32, j as i32));
            }
        }
    }
    elf_locations
}

fn move_elves(
    elf_positions: &mut HashSet<(i32, i32)>,
    proposed_moves: &VecDeque<([(i32, i32); 3], (i32, i32))>,
) -> bool {
    let mut proposed_new_locations_for_elves = HashMap::<(i32, i32), Vec<(i32, i32)>>::new();
    for elf in elf_positions.iter() {
        let mut elf_wants_to_move = false;
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }
                if elf_positions.contains(&(elf.0 + x_offset, elf.1 + y_offset)) {
                    elf_wants_to_move = true;
                }
            }
        }

        let mut proposed_new_location = *elf;
        if elf_wants_to_move {
            for (locations_to_check, target) in proposed_moves.iter() {
                let wants_to_move = !locations_to_check
                    .iter()
                    .any(|l| elf_positions.contains(&(elf.0 + l.0, elf.1 + l.1)));
                if wants_to_move {
                    proposed_new_location = (elf.0 + target.0, elf.1 + target.1);
                    break;
                }
            }
        }
        proposed_new_locations_for_elves
            .entry(proposed_new_location)
            .or_insert(Vec::new())
            .push(*elf);
    }
    let mut new_locations = HashSet::<(i32, i32)>::new();
    for (proposed_new_location, elves) in proposed_new_locations_for_elves {
        if elves.len() == 1 {
            new_locations.insert(proposed_new_location);
        } else {
            for elf in elves {
                // send 'em back home
                new_locations.insert(elf);
            }
        }
    }
    let changed = *elf_positions != new_locations;
    *elf_positions = new_locations;
    changed
}

fn find_containing_rectangle(elf_positions: &HashSet<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
    let some_arbitrary_elf = elf_positions.iter().next().unwrap();
    let mut min_x = some_arbitrary_elf.0;
    let mut max_x = some_arbitrary_elf.0;
    let mut min_y = some_arbitrary_elf.1;
    let mut max_y = some_arbitrary_elf.1;
    for elf in elf_positions {
        if elf.0 < min_x {
            min_x = elf.0;
        }
        if elf.0 > max_x {
            max_x = elf.0;
        }
        if elf.1 < min_y {
            min_y = elf.1;
        }
        if elf.1 > max_y {
            max_y = elf.1;
        }
    }
    ((min_x, min_y), (max_x, max_y))
}

fn find_containing_rectangle_size(elf_positions: &HashSet<(i32, i32)>) -> usize {
    let ((min_x, min_y), (max_x, max_y)) = find_containing_rectangle(&elf_positions);
    ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - elf_positions.len()
}

fn print(elf_positions: &HashSet<(i32, i32)>) {
    let ((min_x, min_y), (max_x, max_y)) = find_containing_rectangle(&elf_positions);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                if elf_positions.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
    println!();
}

fn process(data: &str) -> usize {
    let mut elf_locations = parse_elf_map(data);
    let mut moves = VecDeque::from([
        // If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
        ([(-1, -1), (0, -1), (1, -1)], (0, -1)),
        // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
        ([(-1, 1), (0, 1), (1, 1)], (0, 1)),
        // If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
        ([(-1, 1), (-1, 0), (-1, -1)], (-1, 0)),
        // If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
        ([(1, 1), (1, 0), (1, -1)], (1, 0)),
    ]);
    for _ in 0..10 {
        move_elves(&mut elf_locations, &moves);
        let rotated_move = moves.pop_front().unwrap();
        moves.push_back(rotated_move);
        // print(&elf_locations);
    }
    find_containing_rectangle_size(&elf_locations)
}

fn process2(data: &str) -> usize {
    let mut elf_locations = parse_elf_map(data);
    let mut moves = VecDeque::from([
        ([(-1, -1), (0, -1), (1, -1)], (0, -1)),
        ([(-1, 1), (0, 1), (1, 1)], (0, 1)),
        ([(-1, 1), (-1, 0), (-1, -1)], (-1, 0)),
        ([(1, 1), (1, 0), (1, -1)], (1, 0)),
    ]);
    let mut current_round: usize = 1;
    while move_elves(&mut elf_locations, &moves) {
        current_round += 1;
        let rotated_move = moves.pop_front().unwrap();
        moves.push_back(rotated_move);
    }
    current_round
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process2(data));
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL_DATA: &str = ".....
..##.
..#..
.....
..##.
.....";

    const DATA: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_parse_map() {
        assert_eq!(
            parse_elf_map(SMALL_DATA),
            HashSet::from([(2, 1), (3, 1), (2, 2), (2, 4), (3, 4)])
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(process(DATA), 110);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process2(DATA), 20);
    }
}
