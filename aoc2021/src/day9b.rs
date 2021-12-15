use std::env;
use std::fs;
use std::collections::VecDeque;

fn find_basin_sizes(inputs: Vec<&str>) -> Vec<u32> {
    let mut field: Vec<Vec<bool>> = Vec::new();

    // Create a map of possible basins
    //
    // for a grid like
    //   2199943210
    //   3987894921
    //   9856789892
    //   8767896789
    //   9899965678
    // this will produce a Vec<Vec<bool>> like
    //   ..***.....
    //   .*...*.*..
    //   *.....*.*.
    //   .....*...*
    //   *.***.....
    //
    // Printing this grid can be done with something like the following:
    /*
    for row in &field {
        for cell in row {
            if *cell {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
    */
    let height = inputs.len();
    let width = inputs[0].len();
    let mut remaining_cells = 0;
    for input in inputs {
        let mut row = Vec::new();
        for c in input.chars() {
            let val = c.to_digit(10);
            match val {
                Some(x) => {
                    if x == 9 {
                        row.push(true);
                    } else {
                        remaining_cells += 1;
                        row.push(false);
                    }
                }
                None => {
                    eprintln!("Invalid val '{}'", c);
                    panic!();
                }
            }
        }
        field.push(row);
    }

    // Then walk each basin to find its size
    let mut basin_sizes = Vec::new();
    while remaining_cells > 0 {
        // Find a starting cell for a new basin
        let mut to_visit = VecDeque::new();
        for (x, row) in field.iter().enumerate() {
            for (y, visited) in row.iter().enumerate() {
                if !visited {
                    to_visit.push_back((x, y));
                    break;
                }
            }
            if to_visit.len() > 0 {
                break;
            }
        }

        // Mark our starting cell as visited -- Can't do this in the loop above
        // because it makes the borrow checker unhappy.
        let first_up = to_visit.front().unwrap();
        field[first_up.0][first_up.1] = true;

        let mut new_basin_size = 0;
        // Walk around from that starting cell, incrementing the size for each
        // new cell visited
        while to_visit.len() > 0 {
            let (i,j) = to_visit.pop_front().unwrap();
            remaining_cells -= 1;
            new_basin_size += 1;
            // cell above
            if i != 0 && !field[i-1][j] {
                to_visit.push_back((i-1, j));
                field[i-1][j] = true; // We've been here
            }
            // cell below
            if i != height - 1 && !field[i+1][j] {
                to_visit.push_back((i+1, j));
                field[i+1][j] = true; // We've been here
            }
            // cell to left
            if j != 0 && !field[i][j-1] {
                to_visit.push_back((i, j-1));
                field[i][j-1] = true; // We've been here
            }
            // cell to right
            if j != width - 1 && !field[i][j+1] {
                to_visit.push_back((i, j+1));
                field[i][j+1] = true; // We've been here
            }
        }
        basin_sizes.push(new_basin_size);
    }

    return basin_sizes;

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // no additional args; print help text
        eprintln!("Usage: {} infile.txt", args[0]);
        return;
    }

    let filename = &args[1];
    let raw_inputs = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut basin_sizes = find_basin_sizes(raw_inputs.trim().split("\n").collect());
    basin_sizes.sort();
    basin_sizes.reverse();
    println!("{}", basin_sizes[0..3].iter().product::<u32>());
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str ="
2199943210
3987894921
9856789892
8767896789
9899965678
    ";

    #[test]
    fn test_find_total_risk_level() {
        let mut basin_sizes = find_basin_sizes(INPUT.trim().split("\n").collect());
        basin_sizes.sort();
        basin_sizes.reverse();
        println!("{:?}", basin_sizes);
        assert_eq!(basin_sizes[0..3].iter().product::<u32>(), 1134);
    }
}
