use std::env;
use std::fs;

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for cell in row {
            if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn generate_grid(instructions: &str) -> Vec<Vec<bool>> {
    // how big do we make this dang thing?
    let mut max_x = 0;
    let mut max_y = 0;
    for instruction in instructions.trim().split("\n") {
        let mut coords = instruction.trim().split(",");
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }
    let mut ret = vec![vec![false; max_x+1]; max_y+1];

    // now fill it up
    for instruction in instructions.trim().split("\n") {
        let mut coords = instruction.trim().split(",");
        let x: usize = coords.next().unwrap().parse().unwrap();
        let y: usize = coords.next().unwrap().parse().unwrap();
        ret[y][x] = true;
    }
    return ret;
}

fn process_fold(dots: Vec<Vec<bool>>, fold: &str) -> Vec<Vec<bool>> {
    let fold_target: usize = fold.trim().split("=").nth(1).unwrap().parse().unwrap();
    let fold_direction = fold.trim().chars().nth(11).unwrap();

    // Create new half-sized grid
    let width;
    let height;
    if fold_direction == 'x' {
        // #.##.|#..#.    #####
        // #...#|.....    #...#
        // .....|#...# -> #...#
        // #...#|.....    #...#
        // .#.#.|#.###    #####
        width = fold_target;
        height = dots.len();
    } else {
        assert_eq!(fold_direction, 'y');
        height = fold_target;
        width = dots[0].len();
    }
    let mut ret = vec![vec![false; width + 1]; height + 1];
    
    // copy the base
    for i in 0..height {
        for j in 0..width {
            ret[i][j] = dots[i][j];
        }
    }

    // do the fold
    for i in 0..height {
        for j in 0..width {
            if fold_direction == 'x' {
                if dots[i][2*width - j] {
                    ret[i][j] = true;
                }
            } else {
                if dots[2*height - i][j] {
                    ret[i][j] = true;
                }
            }
        }
    }
    return ret;
}

fn count_visible(grid: &Vec<Vec<bool>>) -> u32{
    let mut visible = 0;
    for row in grid {
        for cell in row {
            if *cell {
                visible += 1;
            }
        }
    }
    return visible;
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

    let mut inputs = raw_input.trim().split("\n\n");
    let points = inputs.next().unwrap();
    let folds = inputs.next().unwrap().trim().split("\n");
    let mut grid = generate_grid(points);

    for fold in folds {
        grid = process_fold(grid, fold);
    }

    print_grid(&grid);
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5";

    #[test]
    fn test_inputs() {
        let mut inputs = INPUT.trim().split("\n\n");
        let points = inputs.next().unwrap();
        let mut folds = inputs.next().unwrap().trim().split("\n");
        let mut grid = generate_grid(points);
        print_grid(&grid);
        assert_eq!(count_visible(&grid), 18);

        grid = process_fold(grid, folds.next().unwrap());
        print_grid(&grid);
        assert_eq!(count_visible(&grid), 17);
        grid = process_fold(grid, folds.next().unwrap());
        print_grid(&grid);
        assert_eq!(count_visible(&grid), 16);
    }

}
