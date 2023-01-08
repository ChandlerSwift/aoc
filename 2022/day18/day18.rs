use std::collections::HashSet;
use std::fs;

// TODO: this should be calculable via something like the number of
// points to the (2/3) power over 2 times a constant I'm too lazy to
// work out for this comment, since that would occur if the points
// formed a rough sphere around the air bubble. But I don't wanna, so
// I'm just setting it to something arbitrary and large.
const MAX_AIR_BUBBLE_SIZE: usize = 10000;

fn is_air_bubble(droplets: &HashSet<(i32, i32, i32)>, start: (i32, i32, i32)) -> bool {
    let mut queue = Vec::new();
    let mut bubble = HashSet::new();
    queue.push(start);
    while queue.len() > 0 {
        if bubble.len() >= MAX_AIR_BUBBLE_SIZE {
            return false;
        }
        let current = queue.pop().unwrap();
        bubble.insert(current);
        for offset in [
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
            (0, 0, 1),
            (0, 1, 0),
            (1, 0, 0),
        ] {
            let neighbor = (
                current.0 + offset.0,
                current.1 + offset.1,
                current.2 + offset.2,
            );
            if !droplets.contains(&neighbor)
                && !queue.contains(&neighbor)
                && !bubble.contains(&neighbor)
            {
                bubble.insert(neighbor);
                queue.push(neighbor);
            }
        }
    }
    true
}

fn process(data: &str, include_air_bubbles: bool) -> usize {
    let droplets: HashSet<(i32, i32, i32)> = data
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
            if !droplets.contains(&neighbor)
                && (include_air_bubbles || !is_air_bubble(&droplets, neighbor))
            {
                surface_area += 1;
            }
        }
    }
    surface_area
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data, false));
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
        assert_eq!(process("1,1,1\n2,1,1", true), 10);
    }

    #[test]
    fn test_part1() {
        assert_eq!(process(DATA, true), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process(DATA, false), 58);
    }
}
