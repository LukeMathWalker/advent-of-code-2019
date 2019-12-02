use itertools::iproduct;
use std::str::FromStr;

fn read_input(path: &str) -> Result<Vec<usize>, anyhow::Error> {
    let input = std::fs::read_to_string(path)?;
    let instructions = input
        .trim()
        .split(",")
        .map(|s| usize::from_str(&s).expect("Failed to parse instruction"))
        .collect();
    Ok(instructions)
}

#[derive(PartialEq, Eq)]
enum Outcome {
    Success,
    Halt,
}

struct TuringMachine {
    memory_tape: Vec<usize>,
    instruction_pointer: usize,
}

impl TuringMachine {
    fn new(memory_tape: Vec<usize>) -> Self {
        assert!(
            memory_tape.len() > 0,
            "There has to be at least one instruction!"
        );
        Self {
            memory_tape,
            instruction_pointer: 0,
        }
    }

    fn execute(mut self) -> Vec<usize> {
        loop {
            let outcome = (&mut self).step();
            if outcome == Outcome::Halt {
                break;
            }
        }
        self.memory_tape
    }

    fn step(&mut self) -> Outcome {
        let opcode = &self.memory_tape[self.instruction_pointer];
        match opcode {
            1 => {
                let lhs_index = self.memory_tape[self.instruction_pointer + 1].clone();
                let rhs_index = self.memory_tape[self.instruction_pointer + 2].clone();
                let output_index = self.memory_tape[self.instruction_pointer + 3].clone();
                let lhs = self.memory_tape[lhs_index].clone();
                let rhs = self.memory_tape[rhs_index].clone();
                let output = lhs + rhs;
                self.memory_tape[output_index] = output;
                self.instruction_pointer += 4;
                Outcome::Success
            }
            2 => {
                let lhs_index = &self.memory_tape[self.instruction_pointer + 1].clone();
                let rhs_index = &self.memory_tape[self.instruction_pointer + 2].clone();
                let output_index = &self.memory_tape[self.instruction_pointer + 3].clone();
                let lhs = &self.memory_tape[*lhs_index].clone();
                let rhs = &self.memory_tape[*rhs_index].clone();
                let output = lhs * rhs;
                self.memory_tape[*output_index] = output;
                self.instruction_pointer += 4;
                Outcome::Success
            }
            99 => Outcome::Halt,
            _ => panic!("Unknown opcode!"),
        }
    }
}

fn reproduce_1202_program_alarm(memory_tape: Vec<usize>) {
    let output_tape = run_program(12, 2, memory_tape);
    println!("Position 0: {:?}", output_tape[0]);
}

fn run_program(noun: usize, verb: usize, mut memory_tape: Vec<usize>) -> Vec<usize> {
    memory_tape[1] = noun;
    memory_tape[2] = verb;

    TuringMachine::new(memory_tape).execute()
}

fn find_input_pair(desired_output: usize, memory_tape: Vec<usize>) -> Option<(usize, usize)> {
    for (noun, verb) in iproduct!(0..=99, 0..=99) {
        let output_tape = run_program(noun, verb, memory_tape.clone());
        if output_tape[0] == desired_output {
            return Some((noun, verb));
        }
    }
    None
}

fn main() -> Result<(), anyhow::Error> {
    let memory_tape = read_input("input.txt")?;

    reproduce_1202_program_alarm(memory_tape.clone());

    if let Some((noun, verb)) = find_input_pair(19690720, memory_tape.clone()) {
        println!("{:?}", 100 * noun + verb);
    }

    Ok(())
}
