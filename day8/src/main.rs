fn main() {
    let input = include_str!("input.txt");
    let grid = parse_input(input);
    let grid_as_slice = grid
        .iter()
        .map(|row| row.as_slice())
        .collect::<Vec<&[u8]>>();

    println!("{:?}", grid_as_slice);
    let res = count_visible_cells(&grid_as_slice);
    println!("{:?}", res);
}

fn count_visible_cells(grid: &Vec<&[u8]>) -> u32 {
    let mut res = 0;
    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, _)| {
            if is_cell_visible_from_outside_grid(grid, (i, j)) {
                res += 1;
            }
        })
    });
    res
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

fn is_cell_visible_from_outside_grid(grid: &Vec<&[u8]>, coordinates: (usize, usize)) -> bool {
    let mut res = (true, true, true, true);
    let (x, y) = coordinates;
    let cell_row: &[u8] = grid.get(x).unwrap();
    let cell: u8 = *cell_row.get(y).unwrap();
    // case right and left
    cell_row.iter().enumerate().for_each(|(i, &c)| {
        if i < y && c >= cell {
            res.0 = false;
        }
        if i > y && c >= cell {
            res.1 = false;
        }
    });
    // case up and down
    grid.iter().enumerate().for_each(|(i, row)| {
        if i < x && row[y] > cell {
            res.2 = false;
        }
        if i > x && row[y] > cell {
            res.3 = false;
        }
    });

    res.0 || res.1 || res.2 || res.3
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cell_visible_from_outside_grid() {
        let test_grid = vec![vec![3, 9, 1], vec![8, 2, 4], vec![5, 6, 7]];
        let test_grid = test_grid
            .iter()
            .map(|row| row.as_slice())
            .collect::<Vec<&[u8]>>();
        assert!(!is_cell_visible_from_outside_grid(&test_grid, (1, 1)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (1, 2)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (2, 1)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (2, 2)));
        assert_eq!(count_visible_cells(&test_grid), 8);
    }

    #[test]
    fn test_aoc_example() {
        let test_grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let test_grid = test_grid
            .iter()
            .map(|row| row.as_slice())
            .collect::<Vec<&[u8]>>();
        assert_eq!(count_visible_cells(&test_grid), 21);
    }

    #[test]
    fn test_is_cell_visible_from_outside_grid_5x5() {
        let test_grid = vec![
            vec![3, 9, 1, 2, 3],
            vec![8, 2, 4, 5, 6],
            vec![5, 6, 7, 8, 9],
            vec![2, 2, 3, 4, 5],
            vec![2, 2, 3, 4, 5],
        ];
        let test_grid = test_grid
            .iter()
            .map(|row| row.as_slice())
            .collect::<Vec<&[u8]>>();
        assert!(!is_cell_visible_from_outside_grid(&test_grid, (1, 1)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (1, 2)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (2, 1)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (2, 2)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (2, 3)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (3, 2)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (3, 3)));
        assert_eq!(count_visible_cells(&test_grid), 24);
    }

    #[test]
    fn test_is_cell_visible_from_outside_grid_10x10() {
        let test_grid = vec![
            vec![3, 9, 1, 2, 3, 4, 5, 6, 7, 8],
            vec![8, 2, 4, 5, 6, 7, 8, 9, 1, 2],
            vec![5, 6, 7, 8, 9, 1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1],
        ];
        let test_grid = test_grid
            .iter()
            .map(|row| row.as_slice())
            .collect::<Vec<&[u8]>>();
        assert!(!is_cell_visible_from_outside_grid(&test_grid, (1, 1)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (1, 2)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (2, 1)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (2, 2)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (2, 3)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (3, 2)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (3, 3)));
        assert!(is_cell_visible_from_outside_grid(&test_grid, (3, 4)));
        assert_eq!(count_visible_cells(&test_grid), 94);
    }
}
