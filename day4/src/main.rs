use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Input should be readable");

    // Part one
    let mut included_section_assignment_count: u32 = 0;
    // Split input into lines
    let lines: Vec<&str> = input.split("\n").collect();

    // Split input into pairs
    let pairs: Vec<(&str, &str)> = lines
        .into_iter()
        .map(|line| line.split_once(",").unwrap())
        .collect();

    // Parse sections into numbers: e.g. "3-10" -> [3, 10]
    let parsed_pairs: Vec<([u32; 2], [u32; 2])> = pairs
        .into_iter()
        .map(|(left_pair, right_pair)| {
            (
                parse_section(left_pair).unwrap(),
                parse_section(right_pair).unwrap(),
            )
        })
        .collect();

    // Go through parsed sections and check if they are included in each other
    for (left_pair, right_pair) in parsed_pairs.clone().into_iter() {
        if check_inclusion(&left_pair, &right_pair) || check_inclusion(&right_pair, &left_pair) {
            included_section_assignment_count += 1;
        }
    }

    println!("Part One Result: {}", included_section_assignment_count);

    // Part two: count the number of overlapping sections
    let mut overlapping_section_assignment_count: u32 = 0;
    for (left_pair, right_pair) in parsed_pairs.clone().into_iter() {
        if check_overlap(&left_pair, &right_pair) {
            overlapping_section_assignment_count += 1;
        }
    }

    println!("Part Two Result: {}", overlapping_section_assignment_count);
}

fn check_inclusion(included_section: &[u32; 2], parent_section: &[u32; 2]) -> bool {
    included_section[0] >= parent_section[0] && included_section[1] <= parent_section[1]
}

fn check_overlap(left_section: &[u32; 2], right_section: &[u32; 2]) -> bool {
    let left_low_overlap =
        left_section[0] <= right_section[0] && left_section[1] >= right_section[0];
    let left_high_overlap =
        left_section[0] <= right_section[1] && left_section[1] >= right_section[1];
    let left_included_overlap =
        left_section[0] >= right_section[0] && left_section[1] <= right_section[1];

    let left_overlap = left_low_overlap || left_high_overlap || left_included_overlap;

    let right_low_overlap =
        right_section[0] <= left_section[0] && right_section[1] >= left_section[0];
    let right_high_overlap =
        right_section[0] <= left_section[1] && right_section[1] >= left_section[1];
    let right_included_overlap =
        right_section[0] >= left_section[0] && right_section[1] <= left_section[1];

    let right_overlap = right_low_overlap || right_high_overlap || right_included_overlap;

    let has_overlap = left_overlap || right_overlap;

    has_overlap
}

fn parse_section(section: &str) -> Option<[u32; 2]> {
    let section = section.split_once("-");
    let first = section?.0.parse::<u32>();
    let second = section?.1.parse::<u32>();
    match (first, second) {
        (Ok(first), Ok(second)) => Some([first, second]),
        _ => None,
    }
}

// Testing util functions
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_overlap() {
        // With overlap on the left
        assert_eq!(check_overlap(&[1, 5], &[3, 7]), true);

        // With no overlap
        assert_eq!(check_overlap(&[1, 5], &[6, 7]), false);

        // With overlap on the right
        assert_eq!(check_overlap(&[3, 7], &[1, 5]), true);

        // With "inclusive" overlap
        assert_eq!(check_overlap(&[1, 7], &[3, 5]), true);
        assert_eq!(check_overlap(&[3, 5], &[1, 7]), true);

        // Edge cases
        assert_eq!(check_overlap(&[1, 5], &[1, 5]), true);
        assert_eq!(check_overlap(&[1, 1], &[2, 2]), false);
        assert_eq!(check_overlap(&[1, 2], &[2, 2]), true);
    }

    #[test]
    fn test_parse_section() {
        let parsed = parse_section("2-5");
        assert_eq!(parsed, Some([2, 5]));

        // Error case
        let parsed = parse_section("2-5-6");
        assert_eq!(parsed, None);
    }

    #[test]
    fn test_check_inclusion() {
        let included = check_inclusion(&[1, 5], &[1, 10]);
        assert_eq!(included, true);

        let right_included = check_inclusion(&[5, 15], &[6, 10]);
        // This is false as check_inclusion checks if the first section is included in the second
        assert_eq!(right_included, false);

        let non_included = check_inclusion(&[1, 5], &[2, 10]);
        assert_eq!(non_included, false);
    }
}
