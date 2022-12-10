#[derive(Clone, PartialEq, Debug)]
struct ElfCPU {
    current_cycle: u32,
    command_stack: Vec<Opcode>,
    x_value: i32,
    max_cycle: u32,
}

#[derive(Clone, PartialEq, Debug)]
struct Opcode {
    command: Command,
    lifespan: u8,
}

#[derive(Clone, PartialEq, Debug)]
enum Command {
    Add(i32),
    Pass,
}

impl ElfCPU {
    fn new() -> Self {
        ElfCPU {
            current_cycle: 1,
            command_stack: Vec::new(),
            x_value: 1,
            max_cycle: 1,
        }
    }

    fn tick(&mut self) {
        let mut is_remove = false;
        if let Some(opcode) = self.command_stack.first_mut() {
            if opcode.lifespan == 0 {
                let value = ElfCPU::get_increment_register_with_command(&opcode.command);
                self.x_value += value;
                is_remove = true;
            } else {
                opcode.lifespan -= 1;
            }
        }
        if is_remove {
            self.command_stack.remove(0);
        }
        // Update cycles
        self.current_cycle += 1;
        self.max_cycle -= 1;
    }

    fn push_opcode(&mut self, opcode: Opcode) {
        self.command_stack.push(opcode);
    }

    pub fn get_increment_register_with_command(command: &Command) -> i32 {
        match command {
            Command::Add(value) => *value,
            Command::Pass => 0,
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let res = part_one(input);
    println!("Part one: {}", res);
}

fn parse_input(input: &str) -> ElfCPU {
    let mut cpu = ElfCPU::new();
    input.lines().for_each(|line| match line {
        "noop" => {
            cpu.push_opcode(Opcode {
                command: Command::Pass,
                lifespan: 0,
            });
            cpu.max_cycle += 1;
        }
        add => {
            if let Some(add_value) = add.strip_prefix("addx ") {
                cpu.push_opcode(Opcode {
                    command: Command::Add(add_value.parse().unwrap()),
                    lifespan: 1,
                });
                cpu.max_cycle += 2;
            }
        }
    });
    cpu
}

fn part_one(input: &str) -> i32 {
    let mut cpu = parse_input(input);
    let mut x_values = vec![];
    let cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    while cpu.max_cycle > 0 {
        cpu.tick();
        if cycles.contains(&(cpu.current_cycle as i32)) {
            x_values.push(cpu.x_value);
        }
    }

    cycles.iter().zip(x_values.iter()).map(|(a, b)| a * b).sum()
}
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    /// Using the AoC example input
    fn integration_test() {
        let input = include_str!("test_input.txt");
        let res = part_one(input);
        assert_eq!(res, 13140);
    }
}
