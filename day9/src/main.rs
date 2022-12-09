use std::collections::HashSet;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("Invalid Move: {0}")]
    InvalidMove(&'a str),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Invalid State: {0}")]
    InvalidState(&'a str),
}

#[derive(Debug, Clone)]
enum MoveCommand {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = include_str!("input.txt");

    // Part One
    let commands: Vec<MoveCommand> = input
        .lines()
        .flat_map(|line| parse_command(line).unwrap())
        .collect();
    let part_one = part_one(commands.clone());
    println!("Part One: {:?}", part_one);

    // Part Two
    let part_two = part_two(commands);
    println!("Part Two: {:?}", part_two);
}

fn parse_command(command: &str) -> Result<Vec<MoveCommand>, Error> {
    let (direction, distance) = match command.split_once(' ') {
        Some((direction, distance)) => (direction, distance),
        None => return Err(Error::InvalidMove(command)),
    };
    let distance: usize = distance.parse::<i32>()? as usize;
    match direction {
        "U" => Ok(vec![MoveCommand::Up; distance]),
        "D" => Ok(vec![MoveCommand::Down; distance]),
        "L" => Ok(vec![MoveCommand::Left; distance]),
        "R" => Ok(vec![MoveCommand::Right; distance]),
        _ => Err(Error::InvalidMove(direction)),
    }
}

fn move_head(move_command: &MoveCommand, head: (i32, i32)) -> (i32, i32) {
    match move_command {
        MoveCommand::Up => (head.0, head.1 + 1),
        MoveCommand::Down => (head.0, head.1 - 1),
        MoveCommand::Left => (head.0 - 1, head.1),
        MoveCommand::Right => (head.0 + 1, head.1),
    }
}

fn move_tail(tail: (i32, i32), head: (i32, i32)) -> Result<(i32, i32), Error<'static>> {
    /*
    If the head is ever two steps directly up, down, left,
    or right from the tail,
    the tail must also move one step in that
    direction so it remains close enough.
    */
    match head.0 - tail.0 {
        2 => match head.1 - tail.1 {
            1 => Ok((tail.0 + 1, tail.1 + 1)),
            -1 => Ok((tail.0 + 1, tail.1 - 1)),
            0 => Ok((tail.0 + 1, tail.1)),
            2 => Ok((tail.0 + 1, tail.1 + 1)),
            -2 => Ok((tail.0 + 1, tail.1 - 1)),
            _ => Err(Error::InvalidState(
                "Tail and Head are not adjacent, where head.x - tail.x == 2",
            )),
        },

        -2 => match head.1 - tail.1 {
            1 => Ok((tail.0 - 1, tail.1 + 1)),
            -1 => Ok((tail.0 - 1, tail.1 - 1)),
            0 => Ok((tail.0 - 1, tail.1)),
            2 => Ok((tail.0 - 1, tail.1 + 1)),
            -2 => Ok((tail.0 - 1, tail.1 - 1)),
            _ => Err(Error::InvalidState(
                "Tail and Head are not adjacent, where head.x - tail.x == -2",
            )),
        },
        0 => match head.1 - tail.1 {
            2 => Ok((tail.0, tail.1 + 1)),
            -2 => Ok((tail.0, tail.1 - 1)),
            0 | 1 | -1 => Ok(tail),
            _ => Err(Error::InvalidState(
                "Tail and Head are not adjacent, where head.x - tail.x == 0",
            )),
        },
        1 => match head.1 - tail.1 {
            1 => Ok(tail),
            -1 => Ok(tail),
            0 => Ok(tail),
            2 => Ok((tail.0 + 1, tail.1 + 1)),
            -2 => Ok((tail.0 + 1, tail.1 - 1)),
            _ => Err(Error::InvalidState(
                "Tail and Head are not adjacent, where head.x - tail.x == 1",
            )),
        },
        -1 => match head.1 - tail.1 {
            1 => Ok(tail),
            -1 => Ok(tail),
            0 => Ok(tail),
            2 => Ok((tail.0 - 1, tail.1 + 1)),
            -2 => Ok((tail.0 - 1, tail.1 - 1)),
            _ => Err(Error::InvalidState(
                "Tail and Head are not adjacent, where head.x - tail.x == -1",
            )),
        },
        _ => Err(Error::InvalidState(
            "Tail and Head are not adjacent, panicking in default",
        )),
    }
}

fn move_snake(
    move_command: &MoveCommand,
    head: (i32, i32),
    tail: (i32, i32),
) -> Result<((i32, i32), (i32, i32)), Error<'static>> {
    let new_head = move_head(move_command, head);
    let new_tail = move_tail(tail, new_head)?;
    Ok((new_head, new_tail))
}

fn part_one(input: Vec<MoveCommand>) -> Result<i32, Error<'static>> {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_visited_cells = HashSet::new();
    tail_visited_cells.insert(tail);
    for command in input {
        let (new_head, new_tail) = move_snake(&command, head, tail)?;
        head = new_head;
        tail = new_tail;
        tail_visited_cells.insert(tail);
    }
    Ok(tail_visited_cells.len() as i32)
}

fn part_two(input: Vec<MoveCommand>) -> Result<i32, Error<'static>> {
    let mut snake = vec![(0, 0); 10];
    let mut snake_visited_cells = HashSet::new();
    snake_visited_cells.insert((0, 0));
    for command in input {
        let mut new_snake = Vec::new();
        let new_head = move_head(&command, snake[0]);
        new_snake.push(new_head);
        for i in 1..snake.len() {
            let new_tail_i = move_tail(snake[i], snake[i - 1])?;
            new_snake.push(new_tail_i);
        }
        snake = new_snake;
        if let Some(tail) = snake.last() {
            snake_visited_cells.insert(*tail);
        }
    }
    Ok(snake_visited_cells.len() as i32)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_str = "R 4\n\
                        U 4\n\
                        L 3\n\
                        D 1\n\
                        R 4\n\
                        D 1\n\
                        L 5\n\
                        R 2\n";
        let commands: Vec<MoveCommand> = input_str
            .lines()
            .flat_map(|line| parse_command(line).unwrap())
            .collect();
        assert_eq!(part_one(commands).unwrap(), 13);
    }

    #[test]
    fn test_part_two() {
        let input_str = "R 4\n\
                        U 4\n\
                        L 3\n\
                        D 1\n\
                        R 4\n\
                        D 1\n\
                        L 5\n\
                        R 2\n";
        let commands: Vec<MoveCommand> = input_str
            .lines()
            .flat_map(|line| parse_command(line).unwrap())
            .collect();
        assert_eq!(part_two(commands).unwrap(), 1);
    }
}
