use std::fs;

use thiserror::Error;

const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSORS: &str = "C";

const LOOSE: &str = "X";
const DRAW: &str = "Y";
const WIN: &str = "Z";

#[derive(Error, Debug)]
pub enum Error {
    #[error("Inputs to duel incorrect")]
    DuelError,
}

fn main() {
    let mut res: u32 = 0;
    let input = fs::read_to_string("src/input.txt").expect("Input file should be readable");

    let lines: Vec<&str> = input.split("\n").collect();

    // split lines into vectors of strings divided by whitespace and execute compute_duel
    lines
        .into_iter()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .for_each(|x| {
            print!("{:?}", x);
            let duel_result = compute_duel_second_part(x[0], x[1]);
            match duel_result {
                Ok(result) => res += result,
                Err(_) => (),
            }
        });
    print!("{}", res);
}

fn compute_duel_second_part(enemy_handshape: &str, outcome: &str) -> Result<u32, Error> {
    let mut result: u32 = 0;
    match enemy_handshape {
        ROCK => match outcome {
            LOOSE => result += 3,
            DRAW => result += 4,
            WIN => result += 8,
            _ => {
                return Err(Error::DuelError);
            }
        },
        PAPER => match outcome {
            LOOSE => result += 1,
            DRAW => result += 5,
            WIN => result += 9,
            _ => {
                return Err(Error::DuelError);
            }
        },
        SCISSORS => match outcome {
            LOOSE => result += 2,
            DRAW => result += 6,
            WIN => result += 7,
            _ => {
                return Err(Error::DuelError);
            }
        },
        _ => {
            return Err(Error::DuelError);
        }
    }
    Ok(result)
}

fn _compute_duel_first_part(enemy_handshape: &str, ally_handshape: &str) -> Result<u32, Error> {
    let mut result: u32 = 0;
    match enemy_handshape {
        ROCK => match ally_handshape {
            "X" => result += 4,
            "Y" => result += 8,
            "Z" => result += 3,
            _ => {
                return Err(Error::DuelError);
            }
        },
        PAPER => match ally_handshape {
            "X" => result += 1,
            "Y" => result += 5,
            "Z" => result += 9,
            _ => {
                return Err(Error::DuelError);
            }
        },
        SCISSORS => match ally_handshape {
            "X" => result += 7,
            "Y" => result += 2,
            "Z" => result += 6,
            _ => {
                return Err(Error::DuelError);
            }
        },
        _ => {
            return Err(Error::DuelError);
        }
    }
    Ok(result)
}
