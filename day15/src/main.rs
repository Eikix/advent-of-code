use rayon::prelude::*;
use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Debug, PartialEq, Eq, Hash)]
enum GridElement {
    Beacon(Point),
    // Sensor is a tuple for sensor_coordinates, nearest_beacon
    Sensor((Point, Point)),
}

fn main() {
    // Part one
    let input = include_str!("input.txt");
    let mut res: HashSet<(i32, i32)> = HashSet::new();
    println!("parsing input...");
    let grid = parse_input(input);
    println!("parsed input..., finding non-beacon areas");
    grid.into_iter().for_each(|element| {
        if let GridElement::Sensor((sensor, beacon)) = element {
            println!("scanning area...");
            let area = get_sensor_area(&sensor, &beacon);
            area.into_iter().for_each(|point| {
                res.insert(point);
            });
        }
    });
    println!("scanned all areas");
    let count = res
        .into_iter()
        .filter(|element| element.0 == 2000000)
        .count();
    println!("{:?}", count);
}

fn manhattan_distance(a: &Point, b: &Point) -> u32 {
    b.0.abs_diff(a.0) + b.1.abs_diff(a.1)
}

fn get_sensor_area<'a>(sensor: &'a Point, nearest_beacon: &'a Point) -> HashSet<Point> {
    // get the manhattan distance between sensor and nearest_beacon
    let distance = manhattan_distance(sensor, nearest_beacon);
    // iterate from 0 to distance
    let res: Vec<Point> = (1..=distance)
        .into_par_iter()
        .map(|d| {
            // iterate over i,j such that i+j = distance, 0 <= i,j <= distance
            (0..=d)
                .flat_map(move |i| {
                    let j = d - i;
                    // reach different non-beacon points by splitting distance = i+j
                    vec![
                        (sensor.0 + i as i32, sensor.1 + j as i32),
                        (sensor.0 - i as i32, sensor.1 - j as i32),
                        (sensor.0 + i as i32, sensor.1 - j as i32),
                        (sensor.0 - i as i32, sensor.1 + j as i32),
                    ]
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .collect();
    let mut area = HashSet::new();
    res.into_iter().for_each(|element| {
        area.insert(element);
    });
    area
}

fn parse_input(input: &str) -> HashSet<GridElement> {
    let mut result = HashSet::new();
    input
        .lines()
        .filter_map(|line| line.split_once(':'))
        .for_each(|(sensor_part, beacon_part)| {
            let sensor_x: i32;
            let sensor_y: i32;
            let mut beacon_x: i32 = 0;
            let mut beacon_y: i32 = 0;
            if let Some(stripped_beacon_part) = beacon_part.strip_prefix(" closest beacon is at ") {
                if let Some((x, y)) = stripped_beacon_part.split_once(", ") {
                    beacon_x = x.strip_prefix("x=").unwrap().parse().unwrap();
                    beacon_y = y.strip_prefix("y=").unwrap().parse().unwrap();
                    let point = (beacon_x, beacon_y);
                    let beacon = GridElement::Beacon(point);
                    result.insert(beacon);
                }
            }
            if let Some(stripped_sensor_part) = sensor_part.strip_prefix("Sensor at ") {
                if let Some((x, y)) = stripped_sensor_part.split_once(", ") {
                    sensor_x = x.strip_prefix("x=").unwrap().parse().unwrap();
                    sensor_y = y.strip_prefix("y=").unwrap().parse().unwrap();
                    let point = (sensor_x, sensor_y);
                    let sensor = GridElement::Sensor((point, (beacon_x, beacon_y)));
                    result.insert(sensor);
                }
            };
        });
    result
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("test_input.txt");
        let grid = parse_input(input);
        // checking a couple of elements exist in the hashset
        let sensor = GridElement::Sensor(((13_i32, 2_i32), (15_i32, 3_i32)));
        assert!(grid.get(&sensor).is_some());

        let beacon = GridElement::Beacon((25_i32, 17_i32));
        assert!(grid.get(&beacon).is_some());

        let sensor_none = GridElement::Sensor(((1_i32, 1_i32), (0_i32, 0_i32)));
        assert!(grid.get(&sensor_none).is_none());
    }

    #[test]
    fn test_get_sensor_area() {
        let sensor = (0_i32, 0_i32);
        let beacon = (3_i32, 0_i32);
        let sensor_area = get_sensor_area(&sensor, &beacon);
        assert_eq!(sensor_area.len(), 24);
    }

    #[test]
    fn test_integration() {
        let mut res: HashSet<(i32, i32)> = HashSet::new();
        let input = include_str!("test_input.txt");
        let grid = parse_input(input);
        grid.into_iter().for_each(|element| {
            if let GridElement::Sensor((sensor, beacon)) = element {
                let area = get_sensor_area(&sensor, &beacon);
                println!("{:?}", area);
                area.into_iter().for_each(|point| {
                    res.insert(point);
                });
            }
        });
        let count = res.into_iter().filter(|element| element.0 == 10).count();
        assert_eq!(count, 26);
    }
}
