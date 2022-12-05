use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid string: {0}")]
    InvalidString(String),
    #[error("invalid number: {0}")]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split the string on the spaces
        let parts: Vec<&str> = s.split(' ').collect();

        // check that the string has the correct number of parts
        if parts.len() != 6 {
            return Err(Error::InvalidString(
                "Input command does not have the correct number of parts".to_string(),
            ));
        }

        // parse the three parts that contain numbers
        let count = parts[1].parse::<usize>()?;
        let from = parts[3].parse::<usize>()? - 1;
        let to = parts[5].parse::<usize>()? - 1;

        Ok(Move { count, from, to })
    }
}

impl Move {
    // Part one
    fn move_stacks(stacks: &mut [Vec<&str>], stack_move: Move) {
        (0..stack_move.count).for_each(|_| {
            if let Some(popped) = stacks[stack_move.from].pop() {
                stacks[stack_move.to].push(popped);
            }
        });
    }

    // Part two
    fn move_stacks_keep_order(stacks: &mut [Vec<&str>], stack_move: Move) {
        let mut temp_stack = Vec::new();
        (0..stack_move.count).for_each(|_| {
            if let Some(popped) = stacks[stack_move.from].pop() {
                temp_stack.push(popped);
            }
        });
        temp_stack.reverse();
        stacks[stack_move.to].append(&mut temp_stack);
    }
}

fn main() {
    // Part one
    let input_stacks = include_str!("input_stacks.txt");
    let input_instructions = include_str!("input_instructions.txt");
    let mut stacks = parse_input_stacks(input_stacks);
    let mut part_two_stacks = stacks.clone();

    let instructions: Vec<Move> = input_instructions
        .lines()
        .map(|line| line.parse::<Move>().unwrap_or_default())
        .collect();

    // move stacks
    instructions
        .clone()
        .into_iter()
        .for_each(|instruction| Move::move_stacks(&mut stacks, instruction));

    let top_of_each_stack = stacks
        .into_iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<Vec<&str>>()
        .join("");

    print!("Part one: {:?}", top_of_each_stack);

    // Part two
    instructions
        .into_iter()
        .for_each(|instruction| Move::move_stacks_keep_order(&mut part_two_stacks, instruction));
    let top_of_each_stack_part_two = part_two_stacks
        .into_iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<Vec<&str>>()
        .join("");
    print!("Part two: {:?}", top_of_each_stack_part_two);
}

fn parse_input_stacks(input: &str) -> Vec<Vec<&str>> {
    let stacks = input
        .split("\n\n")
        .map(|stack| {
            stack
                .lines()
                .map(|stack_element| {
                    stack_element
                        .trim_matches(|element| element == ']' || element == '[' || element == ' ')
                })
                .collect()
        })
        .map(|stack: Vec<&str>| stack.into_iter().rev().collect())
        .collect();
    stacks
}

// Write unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_move() {
        let input = "move 13 from 3 to 9";
        let expected = Move {
            count: 13,
            from: 2,
            to: 8,
        };
        let actual = input.parse::<Move>().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_move_stacks() {
        let mut stacks = vec![vec!["a", "b", "c"], vec!["d", "e", "f"]];
        let move_stacks = Move {
            count: 1,
            from: 0,
            to: 1,
        };
        Move::move_stacks(&mut stacks, move_stacks);
        let expected = vec![vec!["a", "b"], vec!["d", "e", "f", "c"]];
        assert_eq!(expected, stacks);
    }

    #[test]
    fn test_move_stacks_with_multiple_instructions() {
        let mut stacks = vec![vec!["a", "b", "c"], vec!["d", "e", "f"]];
        let move_stacks = vec![
            Move {
                count: 1,
                from: 0,
                to: 1,
            },
            Move {
                count: 2,
                from: 1,
                to: 0,
            },
        ];
        move_stacks
            .into_iter()
            .for_each(|instruction| Move::move_stacks(&mut stacks, instruction));
        let expected = vec![vec!["a", "b", "c", "f"], vec!["d", "e"]];
        assert_eq!(expected, stacks);
    }
}
