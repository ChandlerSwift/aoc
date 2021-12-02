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

    let directions = contents.split('\n');

    let mut x = 0;
    let mut z = 0; // depth
    let mut aim = 0;

    for direction in directions {
        let words = direction.split_whitespace().collect::<Vec<&str>>();
        if words.len() == 0 {
            // empty last line, perhaps
            continue;
        }
        let distance = words[1].parse::<i32>().expect(concat!(
            "Couldn't parse ",
            stringify!(words[1]),
            " as integer"
        ));
        match words[0] {
            "forward" => {x += distance; z += distance * aim},
            "up" => aim -= distance,
            "down" => aim += distance,
            _ => panic!(),
        }
    }
    println!("x{}z{} = {}", x, z, x * z);
}
