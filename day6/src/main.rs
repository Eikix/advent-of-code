use color_eyre::eyre::Result;
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    // read input.txt
    let input = fs::read_to_string("src/input.txt")?;
    part_one(&input);
    part_two(&input);
    Ok(())
}

fn part_one(input: &str) {
    let res: Vec<bool> = input
        .char_indices()
        .map(|(index, _)| input.get(index..index + 4).unwrap_or("aaaa"))
        .map(|chunk| {
            let mut chars = chunk.chars();
            let a = chars.next();
            let b = chars.next();
            let c = chars.next();
            let d = chars.next();
            // make sure a, b, c and d are non-equal
            a != b && a != c && a != d && b != c && b != d && c != d
        })
        .collect();
    // get the first true in res
    if let Some(first_true) = res.iter().position(|&x| x) {
        println!("{:?}", first_true + 4);
    };
}

fn part_two(input: &str) {
    let res: Vec<bool> = input
        .char_indices()
        .map(|(index, _)| input.get(index..index + 14).unwrap_or("aaaaaaaaaaaaaa"))
        .map(|chunk| {
            let mut counts = HashMap::new();
            chunk.chars().for_each(|c| {
                let count = counts.entry(c).or_insert(0);
                *count += 1;
            });
            !counts.into_values().any(|value| value > 1)
        })
        .collect();
    // get the first true in res
    if let Some(first_true) = res.iter().position(|&x| x) {
        println!("{:?}", first_true + 14);
    };
}
