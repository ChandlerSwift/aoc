use std::fs;

#[allow(dead_code)]
fn print_current_order(original_order: &Vec<i64>, positions: &Vec<usize>) {
    println!(
        "{}\n",
        positions
            .iter()
            .map(|p| original_order[*p].to_string())
            .reduce(|a, b| a + ", " + &b)
            .unwrap()
    );
}

fn process(data: &str, decryption_key: i64, mix_count: usize) -> i64 {
    let original_order: Vec<i64> = data
        .split("\n")
        .map(|n| n.parse::<i64>().unwrap() * decryption_key)
        .collect();
    let mut positions: Vec<usize> = (0..original_order.len()).collect();

    for _ in 0..mix_count {
        for i in 0..original_order.len() {
            let current_pos = positions.iter().position(|x| *x == i).unwrap();
            positions.remove(current_pos);
            let new_pos = (current_pos as i64 + original_order[i])
                .rem_euclid(positions.len() as i64) as usize;
            positions.insert(new_pos, i);
        }
    }

    let original_zero_pos = original_order.iter().position(|x| *x == 0).unwrap();
    let start = positions
        .iter()
        .position(|x| *x == original_zero_pos)
        .unwrap();

    println!(
        "{} {} {}",
        original_order[positions[(start + 1000) % positions.len()]],
        original_order[positions[(start + 2000) % positions.len()]],
        original_order[positions[(start + 3000) % positions.len()]]
    );

    original_order[positions[(start + 1000) % positions.len()]]
        + original_order[positions[(start + 2000) % positions.len()]]
        + original_order[positions[(start + 3000) % positions.len()]]
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data, 811589153, 10));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part1() {
        assert_eq!(process(DATA, 1, 1), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process(DATA, 811589153, 10), 1623178306);
    }
}
