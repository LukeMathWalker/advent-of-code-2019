use std::str::FromStr;
use day05::TuringMachine;
use itertools::Itertools;

fn read_input(path: &str) -> Vec<i32> {
    let input = std::fs::read_to_string(path).expect("Failed to read input");
    let instructions = input
        .trim()
        .split(",")
        .map(|s| i32::from_str(&s).expect("Failed to parse instruction"))
        .collect();
    instructions
}

fn amplifiers(settings: impl Iterator<Item=u8>, memory_tape: Vec<i32>) -> i32 {
    let mut input_signal = 0;
    for setting in settings {
        let program = TuringMachine::new(memory_tape.clone());
        let (_, output_tape) = program.execute(vec![setting as i32, input_signal]);
        input_signal = output_tape[0]
    }
    input_signal
}

fn loop_amplifiers(settings: Vec<u8>, memory_tape: Vec<i32>) -> i32 {
    let mut input_signal = 0;
    let mut memory_tapes = vec![
        memory_tape.clone(),
        memory_tape.clone(),
        memory_tape.clone(),
        memory_tape.clone(),
        memory_tape.clone(),
    ];
    for (amplifier_index, setting) in settings.into_iter().enumerate().cycle() {
        println!("Amplifier index: {:?}", amplifier_index);
        let memory_tape = &memory_tapes[amplifier_index];
        let program = TuringMachine::new(memory_tape.to_vec());
        let (memory_tape, output_tape) = program.execute(vec![setting as i32, input_signal]);
        memory_tapes[amplifier_index] = memory_tape;
        if output_tape.len() == 0 {
            break;
        }
        input_signal = output_tape[0]
    }
    input_signal
}


fn main() {
    let memory_tape = read_input("input.txt");

    let mut thrusters_outputs: Vec<i32> = Vec::new();
    let mut looped_thrusters_outputs: Vec<i32> = Vec::new();
    for settings in (0..=4).permutations(5) {
        let thrust = amplifiers(settings.clone().into_iter(), memory_tape.clone());
        thrusters_outputs.push(thrust);
    }

    let optimal_thrust = thrusters_outputs.into_iter().max().unwrap();
    println!("Maximum signal: {:?}", optimal_thrust);

    let mut thrusters_outputs: Vec<i32> = Vec::new();
    for settings in (5..=9).permutations(5) {
        let thrust = loop_amplifiers(settings.clone(), memory_tape.clone());
        thrusters_outputs.push(thrust);
    }

    let optimal_thrust = thrusters_outputs.into_iter().max().unwrap();
    println!("Maximum looped signal: {:?}", optimal_thrust);
}

#[cfg(test)]
mod tests {
    use crate::loop_amplifiers;

    #[test]
    fn loop_amplifiers_test() {
        let memory_tape = vec![
            3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
            27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
        ];
        let thrust = loop_amplifiers(vec![9,8,7,6,5], memory_tape.clone());
        assert_eq!(thrust, 139629729);
    }
}
