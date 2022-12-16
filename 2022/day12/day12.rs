use std::collections::VecDeque;
use std::fs;

fn parse(data: &str) -> (Vec<Vec<i8>>, (usize, usize), (usize, usize)) {
    let mut start = None;
    let mut end = None;
    let mut elevations = Vec::new();
    for (i, line) in data.split("\n").enumerate() {
        let mut row = Vec::new();
        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = Some((i, j));
                row.push(1);
            } else if ch == 'E' {
                end = Some((i, j));
                row.push(26);
            } else {
                row.push(ch as i8 - 'a' as i8);
            }
        }
        elevations.push(row);
    }
    (elevations, start.unwrap(), end.unwrap())
}

fn process(data: &str) -> u32 {
    let (elevations, start, end) = parse(data);
    let map_height = elevations.len();
    let map_width = elevations[0].len();
    let mut costs = vec![vec![u32::MAX; map_width]; map_height];
    costs[start.0][start.1] = 0;

    let mut boundaries = VecDeque::new();
    boundaries.push_back(start);

    while !boundaries.is_empty() {
        let (x, y) = boundaries.pop_front().unwrap();
        let boundary_cost = costs[x][y];

        if x > 0 && elevations[x-1][y] - elevations[x][y] <= 1 {
            let above_boundary_cost = &mut costs[x - 1][y];
            if *above_boundary_cost > boundary_cost + 1 {
                *above_boundary_cost = boundary_cost + 1;
                boundaries.push_back((x - 1, y));
            }
        }
        if x < map_height - 1 && elevations[x+1][y] - elevations[x][y] <= 1 {
            let below_boundary_cost = &mut costs[x + 1][y];
            if *below_boundary_cost > boundary_cost + 1 {
                *below_boundary_cost = boundary_cost + 1;
                boundaries.push_back((x + 1, y));
            }
        }
        if y > 0 && elevations[x][y-1] - elevations[x][y] <= 1 {
            let left_boundary_cost = &mut costs[x][y - 1];
            if *left_boundary_cost > boundary_cost + 1 {
                *left_boundary_cost = boundary_cost + 1;
                boundaries.push_back((x, y - 1));
            }
        }
        if y < map_width - 1 && elevations[x][y+1] - elevations[x][y] <= 1 {
            let right_boundary_cost = &mut costs[x][y + 1];
            if *right_boundary_cost > boundary_cost + 1 {
                *right_boundary_cost = boundary_cost + 1;
                boundaries.push_back((x, y + 1));
            }
        }
    }

    costs[end.0][end.1]
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();

    println!("{}", process(data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small_input() {
        let data = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(process(data), 31);
    }
}
