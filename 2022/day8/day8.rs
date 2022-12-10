use std::fs;

fn is_visible(forest: &Vec<Vec<u8>>, row: usize, column: usize) -> bool {
    let height = forest[row][column];
    let mut hidden_from_north = false;
    for i in 0..row {
	if forest[i][column] >= height {
	    hidden_from_north = true;
	    break;
	}
    }
    let mut hidden_from_south = false;
    for i in row+1..forest.len() {
	if forest[i][column] >= height {
	    hidden_from_south = true;
	    break;
	}
    }
    let mut hidden_from_west = false;
    for i in 0..column {
	if forest[row][i] >= height {
	    hidden_from_west = true;
	    break;
	}
    }
    let mut hidden_from_east = false;
    for i in column+1..forest[0].len() {
	if forest[row][i] >= height {
	    hidden_from_east = true;
	    break;
	}
    }
    !hidden_from_north || !hidden_from_south || !hidden_from_west || !hidden_from_east
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
    let mut visible_count = 0;
    for row in 0..forest.len() {
	for column in 0..forest[0].len() {
	    if is_visible(&forest, row, column) {
		visible_count += 1;
	    }
	}
    }
    visible_count
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
	assert!(process(DATA) == 21);
    }
}
