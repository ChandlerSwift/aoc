use std::fs;

fn scenic_score(forest: &Vec<Vec<u8>>, row: usize, column: usize) -> u32 {
    let height = forest[row][column];
    let mut visible_to_north = 0;
    for i in 1..=row {
	visible_to_north += 1;
	if forest[row-i][column] >= height {
	    break;
	}
    }
    let mut visible_to_south = 0;
    for i in row+1..forest.len() {
	visible_to_south += 1;
	if forest[i][column] >= height {
	    break;
	}
    }
    let mut visible_to_west = 0;
    for i in 1..=column {
	visible_to_west += 1;
	if forest[row][column-i] >= height {
	    break;
	}
    }
    let mut visible_to_east = 0;
    for i in column+1..forest[0].len() {
	visible_to_east += 1;
	if forest[row][i] >= height {
	    break;
	}
    }
    visible_to_north * visible_to_south * visible_to_west * visible_to_east
}

fn process(data: &str) -> u32 {
    let mut forest: Vec<Vec<u8>> = Vec::new();
    for line in data.split("\n") {
	let mut tree_row = Vec::new();
	for c in line.chars() {
	    tree_row.push(c.to_digit(10).unwrap() as u8);
	}
	forest.push(tree_row);
    }
    let mut max = 0;
    for row in 0..forest.len() {
	for column in 0..forest[0].len() {
	    if scenic_score(&forest, row, column) > max {
		max = scenic_score(&forest, row, column);
	    }
	}
    }
    max
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();

    println!("{}", process(data));
}

#[cfg(test)]
mod test {
    use super::*;

    static DATA: &str = "30373
25512
65332
33549
35390";
    
    #[test]
    fn test(){
	assert!(process(DATA) == 8);
    }
}
