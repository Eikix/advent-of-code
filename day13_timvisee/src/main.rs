/* Part one:
 * DISCLAIMER: I DID NOT AUTHOR THIS CODE
 * It was taken from Tim Visee's https://github.com/timvisee/advent-of-code-2022/blob/master/day13a/src/main.rs
 * I wanted to explore new ways to parse complicated inputs. Will add comments to help understand
 * the process
 */

use nom::{alt, char, delimited, map, map_opt, named, separated_list0, separated_pair, tag};
use std::cmp::Ordering;

pub fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .split("\n\n")
            .map(|p| pair(p.as_bytes()).unwrap().1)
            .enumerate()
            .filter(|(_, (a, b))| a.cmp(b) == Ordering::Less)
            .map(|(i, _)| i + 1)
            .sum::<usize>(),
    );

    // part two
    part_two();
}

pub fn part_two() {
    let first = Item::L(vec![Item::L(vec![Item::I(2)])]);
    let second = Item::L(vec![Item::L(vec![Item::I(6)])]);
    let packets: Vec<Item> = include_str!("input.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| item(l.as_bytes()).unwrap().1)
        .filter(|i| i < &second)
        .collect();

    println!(
        "{}",
        (packets.iter().filter(|i| *i < &first).count() + 1) * (packets.len() + 2)
    );
}

#[derive(PartialEq, Debug, Eq)]
enum Item {
    I(u8),
    L(Vec<Item>),
}

// Implementation of an order relation, i.e. how can we say A < B?
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // here Item::I(a) is such that a is of type u8. Type u8 implements the Ord trait and
            // as such, one can call `u8::cmp` on a and b.
            (Item::I(a), Item::I(b)) => u8::cmp(a, b),
            // here we are in the case where a and b are both lists.
            // e.g. [1, 2] vs. [5, 7]
            (Item::L(a), Item::L(b)) => match a.iter().cmp(b) {
                // here we're matching each iterative comparison's result to two outcomes:
                // Less or Greater means that the pair is either in the right order or not,
                // Equal means that
                result if result != Ordering::Equal => result,
                _ => a.len().cmp(&b.len()),
            },
            (Item::I(_), Item::L(b)) if b.len() == 1 => self.cmp(&b[0]),
            (Item::I(a), Item::L(_)) => Item::L(vec![Item::I(*a)]).cmp(other),
            (Item::L(_), Item::I(_)) => other.cmp(self).reverse(),
        }
    }
}
// Implementing PartialOrd is compulsory for implementing Ord.
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// named: Makes a function from a parser combination.
//  // Create a parser that will match either "dragon" or "beast"
// named!( dragon_or_beast, alt!( tag!( "dragon" ) | tag!( "beast" ) ) );

// Given the input "dragon slayer", the parser will match "dragon"
// and the rest will be " slayer"
// let (rest, result) = dragon_or_beast(b"dragon slayer").unwrap();
// assert_eq!(result, b"dragon");
// assert_eq!(rest, b" slayer");

// Given the input "beast of Gevaudan", the parser will match "beast"
// and the rest will be " of Gevaudan"
// let (rest, result) = dragon_or_beast(&b"beast of Gevaudan"[..]).unwrap();
// assert_eq!(result, b"beast");
// assert_eq!(rest, b" of Gevaudan");
//
// alt: Tests a list of parsers one by one until one succeeds.
// first parser: list, computes delimited!(char!('['), separated_list0!(char!(','), item), char!(']')));
// second parser: num, computes map_opt!(nom::character::complete::digit1, atoi::atoi));
//
// map: Maps a function over the result of a parser
// e.g. map!(num, Item::I) will return Item::I(u8)
// as num's return type is u8
named!(pub(crate) item<&[u8], Item>, alt!(map!(list, Item::L) | map!(num, Item::I)));

// map_opt: Applies a function returning an Option over the result of a parser.
// atoi: Parses an integer from a slice.
// Example usage:
// [1, 4, 9] ->
named!(num<&[u8], u8>, map_opt!(nom::character::complete::digit1, atoi::atoi));

named!(list<&[u8], Vec<Item>>, delimited!(char!('['), separated_list0!(char!(','), item), char!(']')));
named!(pair<&[u8], (Item, Item)>, separated_pair!(item, tag!("\n"), item));

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn integration_test() {
        let input = include_str!("input.txt")
            .split("\n\n")
            .map(|p| pair(p.as_bytes()).unwrap().1)
            .collect::<Vec<(Item, Item)>>();
        println!("{:?}", input);
        panic!();
    }
}
