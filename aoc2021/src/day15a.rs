use std::env;
use std::fs;
use std::fmt;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Eq, Clone, Hash, Debug)]
struct Vertex {
    x: i32,
    y: i32,
    distance_from_start: u32,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: is this correct or backwards?
        self.distance_from_start.cmp(&other.distance_from_start)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {} ({}))", self.x, self.y, self.distance_from_start)
    }
}

fn find_best_path_risk(grid: Vec<Vec<u8>>) -> u32 {
    // Using Dijkstra's algorithm
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    println!("Width x height: {}x{}", width, height);
    let mut dist = vec![vec![u32::MAX; width as usize]; height as usize];
    let mut prev: Vec<Vec<Option<Vertex>>> = vec![vec![None; width as usize]; height as usize];

    // I'd love to use a BinaryHeap method, but it doesn't do all the operations
    // I'd need it to do.
    let mut q = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, _cell) in row.iter().enumerate() {
            println!("Inserting at {},{}", j, i);
            q.insert((j, i), Vertex{
                x: j as i32,
                y: i as i32,
                distance_from_start: u32::MAX,
            });
        }
    }
    {
        let mut a = q.get_mut(&(0,0)).unwrap();
        a.distance_from_start = 0;
        dist[0][0] = 0;
    }

    while q.len() > 0 {
        // println!("{:?}", q);

        // find the closest element still in the list
        let mut min_vertex = q.values().next().unwrap().clone(); // Pick an element, any element
        let mut min_vertex_coords = (0, 0);
        for (coords, vertex) in &q {
            if *vertex <= min_vertex {
                min_vertex = vertex.clone();
                min_vertex_coords = *coords;
            }
        }
        // println!("Removing {:?}", min_vertex_coords);
        let u = q.remove(&min_vertex_coords).unwrap();
        println!("Removed {}", u);

        // print out the contents of prev
        for (i, row) in prev.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == None {
                    print!(".");
                } else if i < cell.as_ref().unwrap().y as usize {
                    print!("v");
                } else if i > cell.as_ref().unwrap().y as usize {
                    print!("^");
                } else if j < cell.as_ref().unwrap().x as usize {
                    print!(">");
                } else if j > cell.as_ref().unwrap().x as usize {
                    print!("<");
                } else {
                    print!("*");
                }
            }
            println!();
        }

        if u.x == width - 1 && u.y == height - 1 { // It's our target vertex

            let mut current = u;
            let mut total = 0;
            while !(current.x == 0 && current.y == 0) { // until we've gotten back to the beginning
                total += grid[current.y as usize][current.x as usize] as u32;
                // println!("{}", grid[current.y as usize][current.x as usize] as u32);
                current = Vertex{
                    x: prev[current.y as usize][current.x as usize].as_ref().unwrap().x,
                    y: prev[current.y as usize][current.x as usize].as_ref().unwrap().y,
                    distance_from_start: 0,
                };
            }
            return total;
        }


        let adjacent_vertices = [
            // We're not using distance_from_start here, so /shrug
            Vertex{x: u.x-1, y: u.y, distance_from_start: u32::MAX},
            Vertex{x: u.x+1, y: u.y, distance_from_start: u32::MAX },
            Vertex{x: u.x, y: u.y-1, distance_from_start: u32::MAX },
            Vertex{x: u.x, y: u.y+1, distance_from_start: u32::MAX },
            ];
        for v in adjacent_vertices {
            // println!("Checking {},{}", v.x, v.y);
            if v.x >= 0 && v.x < width && v.y >= 0 && v.y < height && q.contains_key(&(v.x as usize, v.y as usize)) {
                // println!("{} {} {} {}", u, v, dist[u.y as usize][u.x as usize], grid[v.y as usize][v.x as usize]);
                let alt = dist[u.y as usize][u.x as usize] + grid[v.y as usize][v.x as usize] as u32;
                if alt < dist[v.y as usize][v.x as usize] {
                    // println!("Setting {},{} to {}", v.x, v.y, alt);
                    let real_v = q.get_mut(&(v.x as usize, v.y as usize)).unwrap();
                    real_v.distance_from_start = alt;
                    dist[v.y as usize][v.x as usize] = alt;
                    prev[v.y as usize][v.x as usize] = Some(u.clone());
                }
            }
        }
        // println!("{} remaining in Q", q.len());
    }

    panic!("Didn't find a solution!");
}

fn parse_grid(input: String) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();
    for row in input.trim().split("\n") {
        let mut new_row = Vec::new();
        for c in row.trim().chars() {
            new_row.push(c.to_digit(10).unwrap() as u8);
        }
        grid.push(new_row);
    }
    grid
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // no additional args; print help text
        eprintln!("Usage: {} infile.txt", args[0]);
        return;
    }

    let filename = &args[1];
    let raw_input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("{}", find_best_path_risk(parse_grid(raw_input)));
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";

    #[test]
    fn test_find_best_path() {
        assert_eq!(find_best_path_risk(parse_grid(String::from(INPUT))), 40);
    }

    #[test]
    fn test_find_best_path2() {
        assert_eq!(find_best_path_risk(parse_grid(String::from("1991111
        1991991
        1991991
        1991991
        1111991
        9999991
        9999991"))), 20);
    }

    #[test]
    fn test_find_best_path_unequal_dimensions() {
        assert_eq!(find_best_path_risk(parse_grid(String::from("1991111
        1991991
        1991991
        1991991
        1111991
        9999991"))), 19);
    }
}
