// rustup run nightly rustc day15b.rs
#![feature(hash_drain_filter)]

use std::collections::HashSet;
use std::fs;

fn region_in_range(point: (i32, i32), manhattan_distance: i32, region: (i32, i32)) -> bool {
    let top_left_covered =
        (region.0 - point.0).abs() + (region.1 - point.1).abs() <= manhattan_distance;
    let top_right_covered =
        (region.0 + 1000 - point.0).abs() + (region.1 - point.1).abs() <= manhattan_distance;
    let bottom_left_covered =
        (region.0 - point.0).abs() + (region.1 + 1000 - point.1).abs() <= manhattan_distance;
    let bottom_right_covered =
        (region.0 + 1000 - point.0).abs() + (region.1 + 1000 - point.1).abs() <= manhattan_distance;
    return top_left_covered && top_right_covered && bottom_left_covered && bottom_right_covered;
}

fn process(data: &str) -> u64 {
    println!("Populating initial hash set...");
    let mut regions_without_beacon = HashSet::new();
    for x in 0..4000 {
        for y in 0..4000 {
            regions_without_beacon.insert((x * 1000, y * 1000));
        }
    }
    println!("hash set population complete!");

    let mut sensors = Vec::new();

    for sensor in data.split("\n") {
        let (sensor_x, rest) = &sensor[12..].split_once(',').unwrap();
        let sensor_x: i32 = sensor_x.parse().unwrap();
        let (sensor_y, rest) = &rest[3..].split_once(':').unwrap();
        let sensor_y: i32 = sensor_y.parse().unwrap();
        let (beacon_x, rest) = &rest[24..].split_once(',').unwrap();
        let beacon_x: i32 = beacon_x.parse().unwrap();
        let beacon_y: i32 = rest[3..].parse().unwrap();

        let manhattan_distance: i32 = (beacon_x - sensor_x).abs() + (beacon_y - sensor_y).abs();

        sensors.push(((sensor_x, sensor_y), manhattan_distance));

        let drained: HashSet<_> = regions_without_beacon
            .drain_filter(|r| region_in_range((sensor_x, sensor_y), manhattan_distance, *r))
            .collect();

        println!("Drained {}, {} remaining", drained.len(), regions_without_beacon.len());
    }

    let mut found_sensor = None;
    for (i, region) in regions_without_beacon.iter().enumerate() {
        if i % 100 == 0 {
            println!("Scanning regions {}-{}", i, i+99);
        }
        for x in 0..=1000 {
            'region: for y in 0..=1000 {
                for (sensor, distance) in sensors.iter() {
                    if (sensor.0 - region.0 - x).abs() + (sensor.1 - region.1 - y).abs()
                        <= *distance
                    {
                        continue 'region;
                    }
                }
                found_sensor = Some((region.0 + x, region.1 + y));
                println!("Found sensor at  {},{}", region.0 + x, region.1 + y);
            }
        }
    }
    let found_sensor = found_sensor.unwrap();
    found_sensor.0 as u64 * 4_000_000 + found_sensor.1 as u64
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data));
}
