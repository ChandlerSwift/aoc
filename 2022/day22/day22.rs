use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Square {
    Void,
    Wall,
    Space,
}

#[derive(Debug, Eq, PartialEq)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

fn print_map(map: &Vec<Vec<Square>>, position: (i32, i32), orientation: (i32, i32)) {
    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if (j as i32, i as i32) == position {
                match orientation {
                    (1, 0) => print!(">"),
                    (0, -1) => print!("v"),
                    (-1, 0) => print!("<"),
                    (0, 1) => print!("^"),
                    _ => panic!("Illegal orientation"),
                }
            } else {
                match col {
                    Square::Void => print!(" "),
                    Square::Wall => print!("#"),
                    Square::Space => print!("."),
                }
            }
        }
        println!();
    }
}

// Note that this puts a single row of Void around the map, so that I don't overflow
fn parse_map(data: &str) -> Vec<Vec<Square>> {
    let rows: Vec<_> = data.split('\n').collect();
    let max_width = rows.iter().map(|r| r.len()).max().unwrap(); // The first row won't necessarily be full-width, so find one that is
    let mut map = vec![vec![Square::Void; max_width + 2]; rows.len() + 2];
    for (i, row) in rows.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            match c {
                ' ' => (),
                '.' => map[i + 1][j + 1] = Square::Space, // This AoC brought to you by Squarespace. With Squarespace's easy-to-use dra
                '#' => map[i + 1][j + 1] = Square::Wall,
                _ => panic!("Unexpected map char {}", c),
            }
        }
    }
    map
}

fn parse_directions(data: &str) -> Vec<(usize, Option<Rotation>)> {
    let mut directions = Vec::new();
    for l_chunk in data.split('L') {
        let r_chunks: Vec<_> = l_chunk.split('R').collect();
        for r_chunk in &r_chunks[..r_chunks.len() - 1] {
            directions.push((r_chunk.parse().unwrap(), Some(Rotation::Clockwise)));
        }
        directions.push((
            r_chunks[r_chunks.len() - 1].parse().unwrap(),
            Some(Rotation::CounterClockwise),
        ));
    }
    let last_index = directions.len() - 1; // You'd think the borrow checker could figure this out, but it specifically asks me to do this
    directions[last_index].1 = None; // Last one doesn't have an associated rotation
    directions
}

fn process(data: &str) -> usize {
    let (map, directions) = data.split_once("\n\n").unwrap();
    let map = parse_map(map);

    let directions = parse_directions(directions);

    let mut pos = (
        map[1].iter().position(|x| *x == Square::Space).unwrap() as i32,
        1,
    );
    let mut orientation = (1, 0); // facing right
                                  // print_map(&map, pos, orientation);
    for (distance, rotation) in directions {
        for _ in 0..distance {
            let mut target_pos = (pos.0 + orientation.0, pos.1 - orientation.1); // TODO: overflow? Could just make map bigger
            if map[target_pos.1 as usize][target_pos.0 as usize] == Square::Void {
                loop {
                    let next_target_pos =
                        (target_pos.0 - orientation.0, target_pos.1 + orientation.1);
                    if map[next_target_pos.1 as usize][next_target_pos.0 as usize] == Square::Void {
                        break;
                    }
                    target_pos = next_target_pos;
                }
            }
            match map[target_pos.1 as usize][target_pos.0 as usize] {
                Square::Wall => (),
                Square::Space => pos = target_pos,
                Square::Void => panic!("Square is still Void; we just fixed this!"),
            }
        }
        orientation = match rotation {
            None => orientation, // Ignore the last step
            Some(Rotation::Clockwise) => (orientation.1, -orientation.0),
            Some(Rotation::CounterClockwise) => (-orientation.1, orientation.0),
        };
        // println!("{}, {:?}", distance, rotation);
        // print_map(&map, pos, orientation);
    }
    let orientation_val = match orientation {
        (1, 0) => 0,
        (0, -1) => 1,
        (-1, 0) => 2,
        (0, 1) => 3,
        _ => panic!("Illegal orientation"),
    };
    (pos.1 * 1000 + pos.0 * 4 + orientation_val) as usize
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim_end(); // There's some intentional whitespace at the beginning

    println!("{}", process(data));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_parse_directions() {
        assert_eq!(
            parse_directions("10R5L5R10L4R5L5"),
            vec![
                (10, Some(Rotation::Clockwise)),
                (5, Some(Rotation::CounterClockwise)),
                (5, Some(Rotation::Clockwise)),
                (10, Some(Rotation::CounterClockwise)),
                (4, Some(Rotation::Clockwise)),
                (5, Some(Rotation::CounterClockwise)),
                (5, None),
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(process(DATA), 6032);
    }
}
