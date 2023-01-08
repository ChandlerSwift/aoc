use std::collections::HashSet;
use std::fs;

fn process(data: &str) -> usize {
    let droplets: HashSet<(i8, i8, i8)> = data
        .split('\n')
        .map(|line| {
            let mut nums = line.split(',').map(|i| i.parse().unwrap());
            (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .collect();

    let mut surface_area = 0;
    for droplet in droplets.iter() {
        for offset in [
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
            (0, 0, 1),
            (0, 1, 0),
            (1, 0, 0),
        ] {
            let neighbor = (
                droplet.0 + offset.0,
                droplet.1 + offset.1,
                droplet.2 + offset.2,
            );
            if !droplets.contains(&neighbor) {
                surface_area += 1;
            }
        }
    }
    surface_area
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn trivial_part1() {
        assert_eq!(process("1,1,1\n2,1,1"), 10);
    }

    #[test]
    fn test_part1() {
        assert_eq!(process(DATA), 64);
    }
}
