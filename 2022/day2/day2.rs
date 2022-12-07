use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let data = data.trim();

    let mut score = 0;

    for line in data.split("\n") {
        let choices: Vec<&str> = line.split(" ").collect();
        score += match (choices[0], choices[1]) {
            ("A", "X") => 1 + 3,
            ("A", "Y") => 2 + 6,
            ("A", "Z") => 3 + 0,
            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,
            ("C", "Y") => 2 + 0,
            ("C", "Z") => 3 + 3,
            _ => panic!("Unknown combination {:?}", choices),
        };
    }
    println!("{}", score);
}
