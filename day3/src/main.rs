use std::{char, collections::HashMap, fs};

fn main() {
    let first_input = fs::read_to_string("src/input1.txt").expect("Input 1 should be readable");
    let second_input = fs::read_to_string("src/input2.txt").expect("Input 2 should be readable");

    // Part one
    let part_one_rucksacks: Vec<&str> = first_input.split('\n').collect();
    let mut first_result: u32 = 0;
    for rucksack in part_one_rucksacks.into_iter() {
        let (first_compartment, second_compartment) = split_str_in_half(rucksack);
        let first_compartment_occurrence_map = compartment_occurrence_map(first_compartment);
        let second_compartment_occurrence_map = compartment_occurrence_map(second_compartment);
        let priority = compare_occurence_maps(
            &first_compartment_occurrence_map,
            &second_compartment_occurrence_map,
        );
        first_result += priority;
    }
    println!("Part One Result: {}", first_result);

    // Part two

    let mut second_result: u32 = 0;
    let occurence_maps = second_input
        .split('\n')
        .map(compartment_occurrence_map)
        .collect::<Vec<HashMap<char, u32>>>();

    for (i, _) in occurence_maps.iter().enumerate() {
        if i % 3 == 0 {
            if let Some(common_char) = find_common_key_in_maps(&[
                &occurence_maps[i],
                &occurence_maps[i + 1],
                &occurence_maps[i + 2],
            ]) {
                second_result += compute_priority(common_char);
            };
        }
    }
    print!("Part Two Result: {}", second_result);
}

fn split_str_in_half(str: &str) -> (&str, &str) {
    let half = str.len() / 2;
    let (first, second) = str.split_at(half);
    (first, second)
}

fn compartment_occurrence_map(rucksack_compartment: &str) -> HashMap<char, u32> {
    let mut type_occurrence_map: HashMap<char, u32> = HashMap::new();
    for c in rucksack_compartment.chars() {
        let count = type_occurrence_map.entry(c).or_insert(0);
        *count += 1;
    }
    type_occurrence_map
}

fn compare_occurence_maps(
    first_occurence_map: &HashMap<char, u32>,
    second_occurence_map: &HashMap<char, u32>,
) -> u32 {
    let mut result: u32 = 0;
    first_occurence_map.clone().into_keys().for_each(|x| {
        if check_occurence_map(second_occurence_map, x) {
            result += compute_priority(x);
        }
    });
    result
}

fn find_common_key_in_maps(occurence_map_slice: &[&HashMap<char, u32>; 3]) -> Option<char> {
    occurence_map_slice[0].clone().into_keys().find(|&x| {
        check_occurence_map(occurence_map_slice[1], x)
            && check_occurence_map(occurence_map_slice[2], x)
    })
}

fn check_occurence_map(occurence_map: &HashMap<char, u32>, item: char) -> bool {
    occurence_map.keys().any(|&x| x == item)
}

fn compute_priority(item: char) -> u32 {
    match item {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}
