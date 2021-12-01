use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // no additional args; print help text
        eprintln!("Usage: {} infile.txt", args[0]);
        return;
    }

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let depths = contents.split_whitespace();
    let mut prev = 0;
    let mut increasing_depth_count = 0;
    for depth_str in depths {
        let depth = depth_str.parse::<i32>().expect(concat!(
            "Couldn't parse ",
            stringify!(depth_str),
            " as integer"
        ));
        if depth > prev && prev > 0 {
            increasing_depth_count += 1;
        }
        prev = depth;
    }
    println!("{}", increasing_depth_count);
}
