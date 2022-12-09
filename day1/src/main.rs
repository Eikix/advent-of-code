use std::fs;

fn main() {
    // read file "./input.txt"
    let input = fs::read_to_string("src/input.txt").expect("input.txt should be readable");

    // split input into lines
    let lines: Vec<&str> = input.split('\n').collect();

    // split lines into vectors of numbers divided by whitespace
    let mut elves: Vec<Vec<i32>> = Vec::new();
    elves.push(Vec::new());
    let mut counter = 0;
    for line in lines.into_iter() {
        if line.is_empty() {
            counter += 1;
            elves.push(Vec::new());
            continue;
        }
        let line_as_number = line.parse::<i32>();
        match line_as_number {
            Ok(number) => elves[counter].push(number),
            Err(_) => continue,
        }
    }

    let mut result = 0;
    let mut elves_sum: Vec<i32> = elves.into_iter().map(|x| x.iter().sum::<i32>()).collect();

    for _ in 0..3 {
        let max = elves_sum.iter().max().unwrap();
        let maxxed_elf_index = elves_sum.iter().position(|x| x == max).unwrap();
        let max_elf = elves_sum.remove(maxxed_elf_index);
        result += max_elf;
    }

    println!("{}", result);
}
