use std::str::FromStr;
use day05::TuringMachine;

fn read_input(path: &str) -> Vec<i32> {
    let input = std::fs::read_to_string(path).expect("Failed to read input");
    let instructions = input
        .trim()
        .split(",")
        .map(|s| i32::from_str(&s).expect("Failed to parse instruction"))
        .collect();
    instructions
}


fn main() {
    let memory_tape = read_input("input.txt");

    // First part
    let program = TuringMachine::new(memory_tape.clone());
    let (_, output_tape) = program.execute(vec![1]);
    println!("Output tape: {:?}", output_tape);

    // Second part
    let program = TuringMachine::new(memory_tape.clone());
    let (_, output_tape) = program.execute(vec![5]);
    println!("Output tape: {:?}", output_tape);
}
