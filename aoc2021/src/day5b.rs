use std::env;
use std::fs;

#[allow(dead_code)]
fn format_grid(grid: Vec<Vec<i32>>) -> String {
    let mut ret = String::new();
    for row in grid {
        for cell in row {
            ret += &cell.to_string();
            ret += " ";
        }
        ret += "\n";
    }
    return ret;
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
    let inputs = raw_inputs.trim().split("\n");
    let mut field: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    for mut input in inputs {
        input = input.trim();
        let coords = input.split(" -> ").collect::<Vec<&str>>();
        let start = coords[0].split(',').collect::<Vec<&str>>();
        let end = coords[1].split(',').collect::<Vec<&str>>();
        let mut start: Vec<usize> = vec![start[0].parse().unwrap(), start[1].parse().unwrap()];
        let mut end: Vec<usize> = vec![end[0].parse().unwrap(), end[1].parse().unwrap()];
        
        let diagonal = start[0] != end[0] && start[1] != end[1];
        if diagonal {
            if start[0] > end[0] { // always drawing towards positive x
                std::mem::swap(&mut start, &mut end);
            }
            let y_dir: i32;
            if start[1] > end[1] {
                y_dir = -1;
            } else {
                y_dir = 1;
            }
            for i in 0..=end[0]-start[0] {
                let y = (start[1] as i32 + i as i32 * y_dir) as usize;
                let x = start[0]+i;
                field[y][x] += 1;
            }
        } else {
            if start[0] > end[0] || start[1] > end[1]{
                std::mem::swap(&mut start, &mut end);
            }
            for x in start[0]..=end[0] {
                for y in start[1]..=end[1] {
                    field[y][x] += 1;
                }
            }
        }
    }

    let mut total = 0;
    for row in field {
        for cell in row {
            if cell > 1 {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const RAW_MOVES: &str =
        "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

    #[test]
    fn test_apply_puzzle() {
        let inputs = RAW_MOVES.trim().split("\n");
        let mut field: Vec<Vec<i32>> = vec![vec![0; 10]; 10];
        for mut input in inputs {
            input = input.trim();
            let coords = input.split(" -> ").collect::<Vec<&str>>();
            let start = coords[0].split(',').collect::<Vec<&str>>();
            let end = coords[1].split(',').collect::<Vec<&str>>();
            let mut start: Vec<usize> = vec![start[0].parse().unwrap(), start[1].parse().unwrap()];
            let mut end: Vec<usize> = vec![end[0].parse().unwrap(), end[1].parse().unwrap()];
            
            let diagonal = start[0] != end[0] && start[1] != end[1];
            if diagonal {
                if start[0] > end[0] { // always drawing towards positive x
                    std::mem::swap(&mut start, &mut end);
                }
                let y_dir: i32;
                if start[1] > end[1] {
                    y_dir = -1;
                } else {
                    y_dir = 1;
                }
                for i in 0..=end[0]-start[0] {
                    let y = (start[1] as i32 + i as i32 * y_dir) as usize;
                    let x = start[0]+i;
                    field[y][x] += 1;
                }
            } else {
                if start[0] > end[0] || start[1] > end[1]{
                    std::mem::swap(&mut start, &mut end);
                }
                for x in start[0]..=end[0] {
                    for y in start[1]..=end[1] {
                        field[y][x] += 1;
                    }
                }
            }
        }

        let mut total = 0;
        for row in &field {
            for cell in row {
                if *cell > 1 {
                    total += 1;
                }
            }
        }
        println!("{}", format_grid(field));
        assert_eq!(total, 12);
    }
}
