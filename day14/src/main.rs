use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let parsed = parse_input(input);
    let (mut cave_topology, max_depth) = fill_cave_topology(parsed.clone());
    let mut reached_abbyss = false;
    let mut obstacle_created_counter = 0;
    while !reached_abbyss {
        reached_abbyss = sand_fall(&mut cave_topology, max_depth);
        if !reached_abbyss {
            obstacle_created_counter += 1;
        }
    }
    println!("Part one: {}", obstacle_created_counter);

    // part two
    let (mut cave_topology, max_depth) = fill_cave_topology(parsed);
    let mut blocked_entry = false;
    let mut obstacle_created_counter_part_two = 0;
    while !blocked_entry {
        blocked_entry = sand_fall_part_two(&mut cave_topology, max_depth);
        if !blocked_entry {
            obstacle_created_counter_part_two += 1;
        }
    }
    println!("Part two: {}", obstacle_created_counter_part_two);
}

fn parse_input(input: &str) -> Vec<Vec<(u32, u32)>> {
    let points: Vec<Vec<(u32, u32)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .collect::<Vec<&str>>()
                .into_iter()
                .filter_map(|element| element.split_once(','))
                .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
                .collect::<Vec<(u32, u32)>>()
        })
        .collect::<Vec<Vec<(u32, u32)>>>();
    points
}

fn fill_cave_topology(
    topology_coordinates_vec: Vec<Vec<(u32, u32)>>,
) -> (HashSet<(u32, u32)>, u32) {
    let mut cave_topology = HashSet::new();
    let mut max_depth = 0_u32;
    topology_coordinates_vec
        .into_iter()
        .for_each(|topology_coordinates| {
            topology_coordinates
                .clone()
                .into_iter()
                .enumerate()
                .take(&topology_coordinates.len() - 1)
                .for_each(|(index, _)| {
                    // check max_depth
                    max_depth = max_depth.max(u32::max(
                        topology_coordinates[index].1,
                        topology_coordinates[index + 1].1,
                    ));
                    if topology_coordinates[index].0 != topology_coordinates[index + 1].0 {
                        for coordinate in u32::min(
                            topology_coordinates[index].0,
                            topology_coordinates[index + 1].0,
                        )
                            ..=u32::max(
                                topology_coordinates[index].0,
                                topology_coordinates[index + 1].0,
                            )
                        {
                            cave_topology.insert((coordinate, topology_coordinates[index].1));
                        }
                    } else if topology_coordinates[index].1 != topology_coordinates[index + 1].1 {
                        for coordinate in u32::min(
                            topology_coordinates[index].1,
                            topology_coordinates[index + 1].1,
                        )
                            ..=u32::max(
                                topology_coordinates[index].1,
                                topology_coordinates[index + 1].1,
                            )
                        {
                            cave_topology.insert((topology_coordinates[index].0, coordinate));
                        }
                    }
                })
        });
    (cave_topology, max_depth + 2)
}

fn sand_fall_part_two(cave_topology: &mut HashSet<(u32, u32)>, max_depth: u32) -> bool {
    let mut stopped = false;
    let mut coordinates = (500_u32, 1_u32);
    let mut blocked_entry: bool = false;
    while !stopped {
        if cave_topology.get(&(500_u32, 0_u32)).is_some() {
            blocked_entry = true;
            stopped = true;
            continue;
        }
        // dirty solution: check if y coordinate is very high -> meaning you are in the abyss ->
        if coordinates.1 == max_depth {
            cave_topology.insert((coordinates.0, coordinates.1 - 1));
            stopped = true;
            continue;
        }
        if let Some(_) = cave_topology.get(&coordinates) {
            if let Some(_) = cave_topology.get(&(coordinates.0 - 1, coordinates.1)) {
                if let Some(_) = cave_topology.get(&(coordinates.0 + 1, coordinates.1)) {
                    cave_topology.insert((coordinates.0, coordinates.1 - 1));
                    stopped = true;
                } else {
                    coordinates.0 += 1;
                    coordinates.1 += 1;
                }
            } else {
                coordinates.0 -= 1;
                coordinates.1 += 1;
            }
        } else {
            coordinates.1 += 1;
        };
    }
    blocked_entry
}

fn sand_fall(cave_topology: &mut HashSet<(u32, u32)>, max_depth: u32) -> bool {
    let mut stopped = false;
    let mut coordinates = (500_u32, 1_u32);
    let mut reached_abbyss: bool = true;
    while !stopped {
        // dirty solution: check if y coordinate is very high -> meaning you are in the abyss ->
        if coordinates.1 > max_depth ^ 2 {
            cave_topology.insert((coordinates.0, coordinates.1 - 1));
            stopped = true;
        }
        if let Some(_) = cave_topology.get(&coordinates) {
            if let Some(_) = cave_topology.get(&(coordinates.0 - 1, coordinates.1)) {
                if let Some(_) = cave_topology.get(&(coordinates.0 + 1, coordinates.1)) {
                    cave_topology.insert((coordinates.0, coordinates.1 - 1));
                    stopped = true;
                    reached_abbyss = false;
                } else {
                    coordinates.0 += 1;
                    coordinates.1 += 1;
                }
            } else {
                coordinates.0 -= 1;
                coordinates.1 += 1;
            }
        } else {
            coordinates.1 += 1;
        };
    }
    reached_abbyss
}

#[cfg(test)]

mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("test_input.txt");
        let parsed = parse_input(input);
        assert_eq!(
            parsed,
            vec![
                vec![(498, 4), (498, 6), (496, 6)],
                vec![(503, 4), (502, 4), (502, 9), (494, 9)]
            ]
        );
    }

    #[test]
    fn test_fill_cave_topology() {
        let input = include_str!("test_input.txt");
        let parsed = parse_input(input);
        let (cave_topology, _) = fill_cave_topology(parsed);
        assert_eq!(
            cave_topology,
            HashSet::from([
                (498, 4),
                (498, 5),
                (498, 6),
                (497, 6),
                (496, 6),
                (503, 4),
                (502, 4),
                (502, 5),
                (502, 6),
                (502, 7),
                (502, 8),
                (502, 9),
                (501, 9),
                (500, 9),
                (499, 9),
                (498, 9),
                (497, 9),
                (496, 9),
                (495, 9),
                (494, 9)
            ])
        );
    }

    #[test]
    fn test_sand_fall() {
        let input = include_str!("test_input.txt");
        let parsed = parse_input(input);
        let (mut cave_topology, max_depth) = fill_cave_topology(parsed);
        assert!(cave_topology.get(&(500_u32, 8_u32)).is_none());
        sand_fall(&mut cave_topology, max_depth);
        assert!(cave_topology.get(&(500_u32, 8_u32)).is_some())
    }

    #[test]
    fn integration_test() {
        let input = include_str!("test_input.txt");
        let parsed = parse_input(input);
        let (mut cave_topology, max_depth) = fill_cave_topology(parsed);
        let mut reached_abbyss = false;
        let mut counter = 0;
        while !reached_abbyss {
            reached_abbyss = sand_fall(&mut cave_topology, max_depth);
            if !reached_abbyss {
                counter += 1;
            }
        }
        assert_eq!(counter, 24);
    }

    #[test]
    fn integration_test_part_two() {
        let input = include_str!("test_input.txt");
        let parsed = parse_input(input);
        let (mut cave_topology, max_depth) = fill_cave_topology(parsed);
        let mut reached_entry = false;
        let mut counter = 0;
        while !reached_entry {
            reached_entry = sand_fall_part_two(&mut cave_topology, max_depth);
            if !reached_entry {
                counter += 1;
            }
        }
        assert_eq!(counter, 93);
    }
}
