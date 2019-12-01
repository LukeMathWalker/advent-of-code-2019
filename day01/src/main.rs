use anyhow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

/// Given the mass of a module, return the amount of fuel required (1st exercise)
fn compute_naive_fuel(module_mass: u64) -> u64 {
    // Integer division in Rust truncates any fractional part of the exact result
    if module_mass < 6 {
        0
    } else {
        (module_mass / 3) - 2
    }
}

fn compute_crazy_fuel(mut mass: u64) -> u64 {
    let mut total_fuel = 0;
    loop {
        let fuel = compute_naive_fuel(mass);
        if fuel <= 0 {
            break;
        }
        total_fuel += fuel;
        mass = fuel;
    }
    total_fuel
}

fn main() -> Result<(), anyhow::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let modules: Result<Vec<u64>, _> = reader.lines().map(|l| {
        let input = l.expect("Failed to read input line");
        u64::from_str(&input)
    }).collect();
    let modules = modules.expect("Failed to read input.");

    let required_fuel: u64 = modules.iter().map(|m| compute_naive_fuel(*m)).sum();
    println!("The total required fuel is {:?}.", required_fuel);

    let crazy_fuel: u64 = modules.iter().map(|m| compute_crazy_fuel(*m)).sum();
    println!("The total required fuel (including fuel itself) is {:?}.", crazy_fuel);
    Ok(())
}
