use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(parse_char).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn parse_char(c: char) -> Option<u8> {
    let mut map = HashMap::new();

    for (i, letter) in "abcdefghijklmnopqrstuvwxyz".chars().enumerate() {
        map.insert(letter, i as u8);
    }

    // Special case: map 's' to 0 and 'e' to 25
    map.insert('S', 0);
    map.insert('E', 25);

    map.get(&c).cloned()
}

// find start (S) and end (E) positions
fn get_start_end(input: &str) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (i, j);
            } else if c == 'E' {
                end = (i, j);
            }
        }
    }

    (start, end)
}

fn find_shortest_path(
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<u32> {
    let mut distances_to_end: Vec<u32> = Vec::new();
    let mut queue: Vec<((usize, usize), u32)> = Vec::new();
    queue.push((start, 0));
    let mut visited: Vec<Vec<(bool, u32)>> = vec![vec![(false, 0); grid[0].len()]; grid.len()];

    while let Some(((i, j), distance)) = queue.pop() {
        if (i, j) == end {
            distances_to_end.push(distance);
        }

        if visited[i][j].0 && visited[i][j].1 <= distance {
            continue;
        }

        visited[i][j] = (true, distance);

        // check up
        if i > 0 && grid[i - 1][j] <= grid[i][j] + 1 {
            queue.push(((i - 1, j), distance + 1_u32));
        }

        // check down
        if i < grid.len() - 1 && grid[i + 1][j] <= grid[i][j] + 1 {
            queue.push(((i + 1, j), distance + 1_u32));
        }

        // check left
        if j > 0 && grid[i][j - 1] <= grid[i][j] + 1 {
            queue.push(((i, j - 1), distance + 1_u32));
        }

        // check right
        if j < grid[i].len() - 1 && grid[i][j + 1] <= grid[i][j] + 1 {
            queue.push(((i, j + 1), distance + 1_u32));
        }
    }

    distances_to_end.into_iter().min()
}

fn main() {
    // part one
    let input = include_str!("input.txt");
    let grid = parse_input(input);
    let (start, end) = get_start_end(input);
    let result = find_shortest_path(grid.clone(), start, end).unwrap();
    println!("result: {}", result);

    // part two
    let all_possible_start_position = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == 0)
                .map(move |(j, _)| (i, j))
        })
        .collect::<Vec<_>>();
    // compute all possible shortest paths depending on possible start positions

    let result = all_possible_start_position
        .iter()
        .filter_map(|&start| find_shortest_path(grid.clone(), start, end))
        .min()
        .unwrap();
    println!("result: {}", result);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("test_input.txt");
        let parsed = parse_input(input);
        let expected = vec![
            vec![0, 0, 1, 16, 15, 14, 13, 12],
            vec![0, 1, 2, 17, 24, 23, 23, 11],
            vec![0, 2, 2, 18, 25, 25, 23, 10],
            vec![0, 2, 2, 19, 20, 21, 22, 9],
            vec![0, 1, 3, 4, 5, 6, 7, 8],
        ];
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_get_start_end() {
        let input = include_str!("test_input.txt");
        let (start, end) = get_start_end(input);
        assert_eq!(start, (0, 0));
        assert_eq!(end, (2, 5));
    }

    #[test]
    fn integration_test() {
        let input = include_str!("test_input.txt");
        let grid = parse_input(input);
        let (start, end) = get_start_end(input);
        let result = find_shortest_path(grid, start, end).unwrap();
        assert_eq!(result, 31);
    }
}
