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

    let mut depths = contents.split_whitespace();
    let mut increasing_depth_count = 0;
    let mut prevs = [
        depths.next().expect(
            "Input file had <3 entries"
        ).parse::<i32>().expect(concat!(
            "Couldn't parse ",
            stringify!(depth_str),
            " as integer"
        )),
        depths.next().expect(
            "Input file had <3 entries"
        ).parse::<i32>().expect(concat!(
            "Couldn't parse ",
            stringify!(depth_str),
            " as integer"
        )),
        depths.next().expect(
            "Input file had <3 entries"
        ).parse::<i32>().expect(concat!(
            "Couldn't parse ",
            stringify!(depth_str),
            " as integer"
        )),
    ];

    for depth_str in depths {
        let new_depth = depth_str.parse::<i32>().expect(concat!(
            "Couldn't parse ",
            stringify!(depth_str),
            " as integer"
        ));
        if new_depth > prevs[0] {
            increasing_depth_count += 1;
        }
        prevs[0] = prevs[1];
        prevs[1] = prevs[2];
        prevs[2] = new_depth;
    }
    println!("{}", increasing_depth_count);
}
