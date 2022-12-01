use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let data = data.trim();

    let mut highest_totals = vec![0, 0, 0];
    let mut current_total = 0;
    for line in data.split("\n") {
        if line == "" {
            // we've reached the end of an elf's food
            highest_totals.push(current_total);
            highest_totals.sort_by(|a, b| b.cmp(a));
            highest_totals.truncate(3);
            current_total = 0;
        } else {
            current_total += line.parse::<u32>().unwrap();
        }
    }
    highest_totals.push(current_total);
    highest_totals.sort_by(|a, b| b.cmp(a));
    highest_totals.truncate(3);
    println!(
        "{:?}, sum: {}",
        highest_totals,
        highest_totals.iter().sum::<u32>()
    );
}
