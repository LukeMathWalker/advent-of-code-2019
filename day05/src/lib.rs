#[derive(Copy, Clone, Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl From<i32> for ParameterMode {
    fn from(p: i32) -> Self {
        match p {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Invalid parameter mode!")
        }
    }
}

// This would be an awesome spot to leverage const generics!
fn parse_opcode(d: i32) -> (u32, Vec<ParameterMode>) {
    let opcode = d % 100;
    // We don't have any instruction with more than 3 parameters
    let parameter_modes = vec![
        get_digit(d, 3).into(),
        get_digit(d, 4).into(),
        get_digit(d, 5).into(),
    ];
    (opcode as u32, parameter_modes)
}

fn get_digit(n: i32, digit_position: u32) -> i32 {
    (n / 10_i32.pow(digit_position - 1)) % 10
}

#[derive(PartialEq, Eq)]
enum Outcome {
    Success,
    Output(i32),
    Halt,
}

pub struct TuringMachine {
    memory_tape: Vec<i32>,
    instruction_pointer: usize,
}

impl TuringMachine {
    pub fn new(memory_tape: Vec<i32>) -> Self {
        assert!(
            memory_tape.len() > 0,
            "The memory tape cannot be empty!"
        );
        Self {
            memory_tape,
            instruction_pointer: 0,
        }
    }

    pub fn execute(mut self, inputs: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        let mut output_tape = Vec::new();
        let mut inputs = inputs.into_iter();
        loop {
            let outcome = (&mut self).step(&mut inputs);
            if outcome == Outcome::Halt {
                break;
            }
            if let Outcome::Output(output) = outcome {
                output_tape.push(output);
                println!("New output: {:?}", output);
            }
        }
        (self.memory_tape, output_tape)
    }

    fn step(&mut self, inputs: &mut impl Iterator<Item=i32>) -> Outcome {
        let raw_opcode = &self.memory_tape[self.instruction_pointer];
        let (opcode, parameter_modes) = parse_opcode(*raw_opcode);
        println!("Current (opcode, parameter_modes): {:?}, {:?}", opcode, parameter_modes);
        match opcode {
            1 => {
                let lhs = self.get_parameter(1, parameter_modes[0], false);
                let rhs = self.get_parameter(2, parameter_modes[1], false);
                let output_index = self.get_parameter(3, parameter_modes[2], true);
                let output = lhs + rhs;
                println!("Operation output value: {:?}", output);
                self.memory_tape[output_index as usize] = output;
                self.instruction_pointer += 4;
                Outcome::Success
            }
            2 => {
                let lhs = self.get_parameter(1, parameter_modes[0], false);
                let rhs = self.get_parameter(2, parameter_modes[1], false);
                let output_index = self.get_parameter(3, parameter_modes[2], true);
                let output = lhs * rhs;
                println!("Operation output value: {:?}", output);
                self.memory_tape[output_index as usize] = output;
                self.instruction_pointer += 4;
                Outcome::Success
            },
            3 => {
                let output_index = self.get_parameter(1, parameter_modes[0], true);
                let input = inputs.next().expect("Ran out of inputs!");
                self.memory_tape[output_index as usize] = input;
                self.instruction_pointer += 2;
                println!("Operation output value: {:?}", input);
                Outcome::Success
            },
            4 => {
                let output_index = self.get_parameter(1, parameter_modes[0], true);
                let output = self.memory_tape[output_index as usize];
                self.instruction_pointer += 2;
                println!("Operation output value: {:?}", output);
                Outcome::Output(output)
            },
            5 => {
                let first_parameter = self.get_parameter(1, parameter_modes[0], false);
                let second_parameter = self.get_parameter(2, parameter_modes[1], false);
                if first_parameter != 0 {
                    self.instruction_pointer = second_parameter as usize;
                } else {
                    self.instruction_pointer += 3;
                }
                Outcome::Success
            },
            6 => {
                let first_parameter = self.get_parameter(1, parameter_modes[0], false);
                let second_parameter = self.get_parameter(2, parameter_modes[1], false);
                if first_parameter == 0 {
                    self.instruction_pointer = second_parameter as usize;
                } else {
                    self.instruction_pointer += 3;
                }
                Outcome::Success
            },
            7 => {
                let first_parameter = self.get_parameter(1, parameter_modes[0], false);
                let second_parameter = self.get_parameter(2, parameter_modes[1], false);
                let third_parameter = self.get_parameter(3, parameter_modes[2], true);
                if first_parameter < second_parameter {
                    self.memory_tape[third_parameter as usize] = 1;
                } else {
                    self.memory_tape[third_parameter as usize] = 0;
                }
                self.instruction_pointer += 4;
                Outcome::Success
            },
            8 => {
                let first_parameter = self.get_parameter(1, parameter_modes[0], false);
                let second_parameter = self.get_parameter(2, parameter_modes[1], false);
                let third_parameter = self.get_parameter(3, parameter_modes[2], true);
                if first_parameter == second_parameter {
                    self.memory_tape[third_parameter as usize] = 1;
                } else {
                    self.memory_tape[third_parameter as usize] = 0;
                }
                self.instruction_pointer += 4;
                Outcome::Success
            },
            99 => Outcome::Halt,
            _ => panic!("Unknown opcode!"),
        }
    }

    fn get_parameter(&self, position: usize, parameter_mode: ParameterMode, is_output: bool) -> i32 {
        match parameter_mode {
            ParameterMode::Position => {
                let index = &self.memory_tape[self.instruction_pointer + position];
                println!("Parameter {:?}: {:?}", position, index);
                if is_output {
                    *index
                } else {
                    let value = self.memory_tape[*index as usize];
                    println!("Parameter value {:?}: {:?}", position, value);
                    value
                }
            },
            ParameterMode::Immediate => {
                let value = self.memory_tape[self.instruction_pointer + position];
                println!("Parameter value {:?}: {:?}", position, value);
                value
            }
        }
    }
}
