use std::cmp::{max, min};
use std::fs;

fn format_map(map: &Vec<Vec<char>>, offset: usize) -> String {
    let mut ret = String::new();
    for row in map {
        ret.push_str(&row.iter().skip(offset).collect::<String>());
        ret.push('\n');
    }
    ret
}

fn parse_map(data: &str) -> Vec<Vec<char>> {
    // find max dimensions
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for line in data.split("\n") {
        for point in line.split(" -> ") {
            let mut coord_pair = point.split(",");
            let x = coord_pair.next().unwrap().parse().unwrap();
            let y = coord_pair.next().unwrap().parse().unwrap();
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }

    let mut map = vec![vec!['.'; max_x + 1]; max_y + 1];

    for line in data.split("\n") {
        let points: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|c| {
                let mut coords = c.split(",").map(|p| p.parse().unwrap());
                (coords.next().unwrap(), coords.next().unwrap())
            })
            .collect();
        for i in 0..points.len() - 1 {
            let first = points[i];
            let second = points[i + 1];
            if first.0 != second.0 {
                // vertical wall
                assert_eq!(first.1, second.1); // No diagonal walls!
                let start = min(first.0, second.0);
                let finish = max(first.0, second.0);
                for j in start..=finish {
                    map[first.1][j] = '#';
                }
            } else {
                // horizontal wall
                assert_eq!(first.0, second.0);
                let start = min(first.1, second.1);
                let finish = max(first.1, second.1);
                for j in start..=finish {
                    map[j][first.0] = '#';
                }
            }
        }
    }

    map
}

// returns the active sand (if it hasn't fallen off the map)
fn tick(map: &mut Vec<Vec<char>>, active_sand: &(usize, usize)) -> Option<(usize, usize)> {
    if active_sand.1 == map.len() - 1 {
        return None; // We fell off the bottom of the map!
    }
    for x_offset in vec![0, -1, 1] {
        // first try down, then down-left, then down-right
        if map[active_sand.1 + 1][active_sand.0.checked_add_signed(x_offset).unwrap()] == '.' {
            return Some((
                active_sand.0.checked_add_signed(x_offset).unwrap(),
                active_sand.1 + 1,
            ));
        }
    }
    // If we got here, we can't move, so we stay put:
    Some(*active_sand)
}

fn process(map: &mut Vec<Vec<char>>, sand_source: (usize, usize)) -> usize {
    let mut sand_count = 0;
    let mut active_sand = sand_source;
    loop {
        match tick(map, &active_sand) {
            Some(new_active_sand) => {
                if new_active_sand == active_sand {
                    // This sand has come to rest; let's add more sand
                    map[active_sand.1][active_sand.0] = 'o';
                    active_sand = sand_source;
                    sand_count += 1;
                } else {
                    active_sand = new_active_sand;
                }
            }
            None => break, // Sand is falling off the map, so we're done
        }
    }
    sand_count
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(&mut parse_map(data), (500, 0)));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_parsing() {
        let goal = "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.";
        let mut map = parse_map(DATA);
        map[0][500] = '+';
        assert_eq!(format_map(&map, 494).trim(), goal);
    }

    #[test]
    fn test_part1() {
        assert_eq!(process(&mut parse_map(DATA), (500, 0)), 24);
    }
}
