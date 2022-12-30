use std::collections::HashSet;
use std::fs;

fn process(data: &str, target_row: i32) -> usize {
    let mut locations_without_beacon = HashSet::new();
    let mut beacons_on_target_row = HashSet::new();

    for sensor in data.split("\n") {
        let (sensor_x, rest) = &sensor[12..].split_once(',').unwrap();
        let sensor_x: i32 = sensor_x.parse().unwrap();
        let (sensor_y, rest) = &rest[3..].split_once(':').unwrap();
        let sensor_y: i32 = sensor_y.parse().unwrap();
        let (beacon_x, rest) = &rest[24..].split_once(',').unwrap();
        let beacon_x: i32 = beacon_x.parse().unwrap();
        let beacon_y: i32 = rest[3..].parse().unwrap();
        if beacon_y == target_row {
            beacons_on_target_row.insert(beacon_x);
        }

        let manhattan_distance: i32 = (beacon_x - sensor_x).abs() + (beacon_y - sensor_y).abs();

        let remaining_width = manhattan_distance - (sensor_y - target_row).abs();
        for c in (sensor_x - remaining_width)..=(sensor_x + remaining_width) {
            locations_without_beacon.insert(c);
        }
    }

    for beacon in beacons_on_target_row {
        locations_without_beacon.remove(&beacon);
    }

    locations_without_beacon.len()
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data, 2_000_000));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(process(DATA, 10), 26);
    }
}
