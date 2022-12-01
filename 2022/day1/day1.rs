use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");
    let data = data.trim();

    let mut highest_total = 0;
    let mut current_total = 0;
    for line in data.split("\n") {
        if line == "" {
            // we've reached the end of an elf's food
            if current_total > highest_total {
                highest_total = current_total;
            }
            current_total = 0;
        } else {
            current_total += line.parse::<u32>().unwrap();
        }
    }
    if current_total > highest_total {
        highest_total = current_total;
    }
    println!("{}", highest_total);
}
