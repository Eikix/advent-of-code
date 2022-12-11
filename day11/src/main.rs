#[derive(Clone, PartialEq, Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    inspection_counts: Vec<u32>,
}

impl Monkeys {
    fn new() -> Self {
        Self {
            monkeys: Vec::new(),
            inspection_counts: Vec::new(),
        }
    }

    fn insert_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
        self.inspection_counts.push(0);
    }

    fn play_round(&mut self) {
        for index in 0..self.monkeys.len() {
            let mut new_items: Vec<(usize, u32)> = Vec::new();
            for item in self.monkeys[index].items.iter() {
                let inspected_item = self.monkeys[index].inspect(*item);

                // increment inspection count
                self.inspection_counts[index] += 1;

                // get monkey to throw to
                let target_monkey = self.monkeys[index].throw_target(inspected_item);
                new_items.push((target_monkey as usize, inspected_item));
            }

            // Clearing monkey's items
            self.monkeys[index].items.clear();

            for (target_monkey, item) in new_items {
                // Get a mutable reference to the target monkey
                let target_monkey = &mut self.monkeys[target_monkey];

                // Add the item to the target monkey's items
                target_monkey.items.push(item);
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: Vec<u32>,
    inspection_factor: (char, u32),
    divisible_condition_factor: u32,
    // If true, throw item to target_monkeys.0, else target_monkeys.1
    target_monkeys: (u8, u8),
}

impl Monkey {
    fn inspect(&self, item: u32) -> u32 {
        match self.inspection_factor.0 {
            '+' => {
                // quick and dirty way to implement new = old + old / old * old
                if self.inspection_factor.1 == 0 {
                    (item + item) / 3
                } else {
                    (item + self.inspection_factor.1) / 3
                }
            }
            '*' => {
                // quick and dirty way to implement new = old + old / old * old
                if self.inspection_factor.1 == 0 {
                    (item * item) / 3
                } else {
                    (item * self.inspection_factor.1) / 3
                }
            }
            _ => panic!("Invalid inspection factor"),
        }
    }

    fn divisible(&self, item: u32) -> bool {
        item % self.divisible_condition_factor == 0
    }

    fn throw_target(&self, inspected_item: u32) -> u8 {
        if self.divisible(inspected_item) {
            self.target_monkeys.0
        } else {
            self.target_monkeys.1
        }
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let mut lines = input.lines().skip(1);
    let items: Vec<u32> = lines
        .next()
        .unwrap()
        .strip_prefix("  Starting items: ")
        .unwrap()
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect();
    let inspection_factor_unparsed: (&str, &str) = lines
        .next()
        .unwrap()
        .strip_prefix("  Operation: new = old ")
        .unwrap()
        .split_once(' ')
        .unwrap();

    let inspection_mul_factor: (char, u32) = (
        inspection_factor_unparsed
            .0
            .chars()
            .collect::<Vec<char>>()
            .first()
            .unwrap()
            .to_owned(),
        inspection_factor_unparsed.1.parse().unwrap_or_default(),
    );
    let divisible_condition_factor = lines
        .next()
        .unwrap()
        .strip_prefix("  Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();
    let first_target_monkey = lines
        .next()
        .unwrap()
        .strip_prefix("    If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();
    let second_target_monkey = lines
        .next()
        .unwrap()
        .strip_prefix("    If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();
    Monkey {
        items,
        inspection_factor: inspection_mul_factor,
        divisible_condition_factor,
        target_monkeys: (first_target_monkey, second_target_monkey),
    }
}

fn parse_input(input: &str) -> Monkeys {
    let mut monkeys = Monkeys::new();
    let input = input.split("\n\n");
    for monkey_input in input {
        monkeys.insert_monkey(parse_monkey(monkey_input));
    }
    monkeys
}

fn main() {
    let input = include_str!("input.txt");
    let mut monkeys = parse_input(input);
    for _ in 0..20 {
        monkeys.play_round();
    }
    let mut inspection_counts = monkeys.inspection_counts.clone();
    inspection_counts.sort();
    inspection_counts.reverse();
    let highest = inspection_counts[0];
    let second_highest = inspection_counts[1];
    println!("Total inspection count: {}", highest * second_highest);
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_parse_monkey() {
        let input = include_str!("test_input.txt")
            .split("\n\n")
            .collect::<Vec<&str>>();
        let first_input = input[0];
        let monkey = parse_monkey(first_input);
        assert_eq!(
            monkey,
            Monkey {
                items: vec![79, 98],
                inspection_factor: ('*', 19),
                divisible_condition_factor: 23,
                target_monkeys: (2, 3),
            }
        );
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("test_input.txt");
        let monkeys = parse_input(input);
        assert_eq!(
            monkeys.monkeys[0],
            Monkey {
                items: vec![79, 98],
                inspection_factor: ('*', 19),
                divisible_condition_factor: 23,
                target_monkeys: (2, 3),
            }
        );
        assert_eq!(
            monkeys.monkeys[1],
            Monkey {
                items: vec![54, 65, 75, 74],
                inspection_factor: ('+', 6),
                divisible_condition_factor: 19,
                target_monkeys: (2, 0),
            }
        );
        assert_eq!(
            monkeys.monkeys[2],
            Monkey {
                items: vec![79, 60, 97],
                inspection_factor: ('*', 0),
                divisible_condition_factor: 13,
                target_monkeys: (1, 3),
            }
        );
        assert_eq!(
            monkeys.monkeys[3],
            Monkey {
                items: vec![74],
                inspection_factor: ('+', 3),
                divisible_condition_factor: 17,
                target_monkeys: (0, 1),
            }
        );
    }

    #[test]
    fn test_integration() {
        let input = include_str!("test_input.txt");
        let mut monkeys = parse_input(input);
        for _ in 0..20 {
            monkeys.play_round()
        }
        // get the two highest values of inspection counts
        let mut inspection_counts = monkeys.inspection_counts.clone();
        inspection_counts.sort();
        inspection_counts.reverse();
        let highest = inspection_counts[0];
        let second_highest = inspection_counts[1];
        assert_eq!(highest, 105);
        assert_eq!(second_highest, 101);
        assert_eq!(highest * second_highest, 10605)
    }
}
