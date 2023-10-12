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

// Note that this puts a single row of Void around the map, so that we
// don't overflow
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

fn process(data: &str, cube: bool) -> usize {
    let (map, directions) = data.split_once("\n\n").unwrap();
    let map = parse_map(map);
    let directions = parse_directions(directions);

    let mut pos = (
        map[1].iter().position(|x| *x == Square::Space).unwrap() as i32,
        1,
    );
    let mut orientation = (1, 0); // facing right

    for (distance, rotation) in directions {
        for _ in 0..distance {
            // We can do this because the map has a one-cell buffer on
            // each edge, so we won't have any out-of-bounds-ness here.
            let mut target_pos = (pos.0 + orientation.0, pos.1 - orientation.1);
            let mut target_orientation = orientation.clone();

            if map[target_pos.1 as usize][target_pos.0 as usize] == Square::Void {
                if cube {
                    // Unfortunately, I haven't managed to find a way
                    // to generalize this problem, so we're going to
                    // end up hardcoding a bunch of things
                    // here. Beyond just being a bit ugly and
                    // non-generalizable, this also means my test case
                    // won't work, since that uses a different cube
                    // net.
                    //
                    // That said, here's some more neat looking cube
                    // nets I drew!
                    //     ┌─1─┐
                    //     3   2
                    // ┌─3─┼───┼─2─┬─1─┐
                    // 4   │   │   │   4
                    // └─5─┼───┼─7─┴─6─┘
                    //     5   7
                    //     └─6─┘
                    // ┌───┬───┬───┐
                    // │   │   │   │
                    // └───┴───┼───┼───┬───┐
                    //         │   │   │   │
                    //         └───┴───┴───┘
                    //
                    // All the numbers are slightly different than
                    // they'd otherwise be since I have an extra
                    // padding square surrounding the board.
                    //
                    // Here's the shape we'll be using, with connected
                    // sides labeled:
                    //       ┌─1─┬─2─┐
                    //       6   │   3
                    //       ├───┼─4─┘
                    //       7   4
                    //   ┌─7─┼───┤
                    //   6   │   3
                    //   ├───┼─5─┘
                    //   1   5
                    //   └─2─┘
                    //
                    match target_pos {
                        // side labeled 1
                        (x, 0) if x > 50 && x <= 100 => {
                            target_pos = (1, x + 100);
                            target_orientation = (1, 0);
                        }
                        (0, y) if y > 150 && y <= 200 => {
                            target_pos = (y - 100, 1);
                            target_orientation = (0, -1);
                        }
                        // 2
                        (x, 0) if x > 100 && x <= 150 => {
                            target_pos = (x - 100, 200);
                            // orientation unchanged
                        }
                        (x, 201) if x > 0 && x <= 50 => {
                            target_pos = (x + 100, 1);
                            // orientation unchanged
                        }
                        // 3
                        (151, y) if y > 0 && y <= 50 => {
                            target_pos = (100, 151 - y);
                            target_orientation = (-1, 0);
                        }
                        (101, y) if y > 100 && y <= 150 => {
                            target_pos = (150, 151 - y);
                            target_orientation = (-1, 0);
                        }
                        // 4
                        (x, 51) if x > 100 && x <= 150 => {
                            target_pos = (100, x - 50);
                            target_orientation = (-1, 0);
                        }
                        (101, y) if y > 50 && y <= 100 => {
                            target_pos = (y + 50, 50);
                            target_orientation = (0, 1);
                        }
                        // 5
                        (x, 151) if x > 50 && x <= 100 => {
                            target_pos = (50, 100 + x);
                            target_orientation = (-1, 0);
                        }
                        (51, y) if y > 150 && y <= 200 => {
                            target_pos = (y - 100, 150);
                            target_orientation = (0, 1);
                        }
                        // 6
                        (0, y) if y > 100 && y <= 150 => {
                            target_pos = (51, 151 - y);
                            target_orientation = (1, 0);
                        }
                        (50, y) if y > 0 && y <= 50 => {
                            target_pos = (1, 151 - y);
                            target_orientation = (1, 0);
                        }
                        // 7
                        (x, 100) if x > 0 && x <= 50 => {
                            target_pos = (51, x + 50);
                            target_orientation = (1, 0);
                        }
                        (50, y) if y > 50 && y <= 100 => {
                            target_pos = (y - 50, 101);
                            target_orientation = (0, -1);
                        }
                        _ => panic!("I haven't handled this case: {:?}", target_pos),
                    }
                } else {
                    loop {
                        let next_target_pos =_pos.0 - orientation.0, target_pos.1 + orientation.1);
                        if map[next_target_pos.1 as usize][next_target_pos.0 as usize] == Square::Void {
                            break;
                        }
                        target_pos = next_target_pos;
                    }
                }
            }
            match map[target_pos.1 as usize][target_pos.0 as usize] {
                Square::Wall => (),
                Square::Space => {
                    pos = target_pos;
                    orientation = target_orientation;
                }
                Square::Void => panic!("Square is still Void; we just fixed this!"),
            }
        }
        orientation = match rotation {
            None => orientation, // Ignore the last step
            Some(Rotation::Clockwise) => (orientation.1, -orientation.0),
            Some(Rotation::CounterClockwise) => (-orientation.1, orientation.0),
        };
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
    println!("{}", process(data, true));
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
        assert_eq!(process(DATA, false), 6032);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process(DATA, true), 5031);
    }
}
