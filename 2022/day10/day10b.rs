use std::fs;

fn process(data: &str) -> String {
    let mut current_cycle = 0; // This is a bit different from the docs, but zero-indexing here makes things easier later
    let mut sprite_position: i32 = 1;
    let mut output = String::new();
    for line in data.split("\n") {
        let instruction: Vec<&str> = line.split(" ").collect();
        match instruction[0] {
            "noop" => {
                output.push(if (sprite_position - (current_cycle % 40)).abs() <= 1 {
                    '#'
                } else {
                    '.'
                });
                current_cycle += 1;
                if current_cycle % 40 == 0 {
                    output.push('\n');
                }
            }
            "addx" => {
                output.push(if (sprite_position - (current_cycle % 40)).abs() <= 1 {
                    '#'
                } else {
                    '.'
                });
                current_cycle += 1;
                if current_cycle % 40 == 0 {
                    output.push('\n');
                }
                output.push(if (sprite_position - (current_cycle % 40)).abs() <= 1 {
                    '#'
                } else {
                    '.'
                });
                current_cycle += 1;
                if current_cycle % 40 == 0 {
                    output.push('\n');
                }
                sprite_position += instruction[1].parse::<i32>().unwrap();
            }
            _ => panic!("Unknown instruction {}", instruction[0]),
        }
    }
    output.trim().to_string()
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data));
}

#[cfg(test)]
mod test {
    use super::*;

    static DATA: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test() {
        println!("{}", process(DATA));
        assert!(
            process(DATA)
                == "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                    .to_string()
        );
    }
}
