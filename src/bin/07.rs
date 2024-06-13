advent_of_code::solution!(7);
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_signal_value(
    signal: &str,
    wires_and_signals: &mut HashMap<String, u32>,
    instructions: &HashMap<String, Vec<String>>,
) -> u32 {
    match signal.parse::<u32>() {
        // If parsing it as a u32 worked, it was a number:
        Ok(value) => value,

        // Otherwise, it was a wire identifier:
        Err(_) => {
            if !wires_and_signals.contains_key(signal) {
                let value = instructions.get(signal).map_or(0, |instruction| {
                    calculate_signal(instruction, wires_and_signals, instructions)
                });
                wires_and_signals.insert(signal.to_string(), value);
            }
            *wires_and_signals.get(signal).unwrap()
        }
    }
}

fn calculate_signal(
    instruction: &Vec<String>,
    wires_and_signals: &mut HashMap<String, u32>,
    instructions: &HashMap<String, Vec<String>>,
) -> u32 {
    match instruction.as_slice() {
        // One word: a signal we can assign directly
        [wire] => get_signal_value(wire, wires_and_signals, instructions),

        // Two words: "NOT <wire>"
        [_, other_wire] => !get_signal_value(other_wire, wires_and_signals, instructions),

        // Three words: wire/signal AND/OR wire/signal, or wire/signal LSHIFT/RSHIFT number
        [left_operand, operator, right_operand] => {
            let signal1 = get_signal_value(left_operand, wires_and_signals, instructions);
            let signal2 = get_signal_value(right_operand, wires_and_signals, instructions);
            match operator.as_str() {
                "AND" => signal1 & signal2,
                "OR" => signal1 | signal2,
                "LSHIFT" => signal1 << signal2,
                "RSHIFT" => signal1 >> signal2,
                _ => panic!("Unknown operator: {}", operator),
            }
        }
        _ => panic!("Invalid instruction: {:?}", instruction),
    }
}

// Define a recursive function to visit a wire
fn visit(
    wire: &str,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    sorted_wires: &mut Vec<String>,
) {
    if !visited.contains(wire) {
        visited.insert(wire.to_string());

        if let Some(dependencies) = graph.get(wire) {
            for dependency in dependencies {
                visit(dependency, graph, visited, sorted_wires);
            }
        }

        sorted_wires.push(wire.to_string());
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    // Create a HashMap to store the dependency graph
    let mut dependency_graph: HashMap<String, Vec<String>> = HashMap::new();

    // Iterate over each line in the input
    for line in input.lines() {
        // Split the line into parts
        let mut parts = line.split("->");
        let instruction: Vec<&str> = parts.next().unwrap().split_whitespace().collect();
        let target = parts.last().unwrap().trim();

        let operators = ["AND", "OR", "LSHIFT", "RSHIFT", "NOT"];

        // Iterate over each wire in the instruction
        for &wire in &instruction {
            // Skip if the wire is a number or an operator
            if wire.parse::<u32>().is_err() && !operators.contains(&wire) {
                // Add a dependency from the target to the wire
                dependency_graph
                    .entry(target.to_string())
                    .or_insert_with(Vec::new)
                    .push(wire.to_string());
            }
        }
    }

    // Create a HashSet to store the visited wires
    let mut visited: HashSet<String> = HashSet::new();

    // Create a Vec to store the sorted wires
    let mut sorted_wires: Vec<String> = Vec::new();

    // Visit each wire in the dependency graph
    for wire in dependency_graph.keys() {
        visit(wire, &dependency_graph, &mut visited, &mut sorted_wires);
    }

    // Reverse the sorted wires to get the correct order
    sorted_wires.reverse();

    // Create a HashMap to store the instructions for each wire
    let mut instructions: HashMap<String, Vec<String>> = HashMap::new();

    // Store the instructions for each wire
    input.lines().for_each(|line| {
        let mut parts = line.split("->");
        let instruction: Vec<String> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(String::from)
            .collect();
        let target = parts.last().unwrap().trim().to_string();
        instructions.insert(target, instruction);
    });

    // Calculate the signal values in the correct order
    let mut wires_and_signals: HashMap<String, u32> = HashMap::new();

    for wire in sorted_wires {
        if let Some(instruction) = instructions.get(&wire) {
            let result = calculate_signal(instruction, &mut wires_and_signals, &instructions);
            wires_and_signals.insert(wire, result);
        }
    }

    let wires_and_signals: BTreeMap<String, u32> = BTreeMap::from_iter(wires_and_signals);

    // Now you can iterate over wires_and_signals, and the keys will be sorted
    for (key, value) in &wires_and_signals {
        println!("{}: {}", key, value);
    }
    Some(*wires_and_signals.get("a").unwrap())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {
        let examples = vec![("abc", Some(1)), ("def", Some(1)), ("ghi", Some(1))];

        for (input, expected) in examples {
            let result = part_two(input);
            assert_eq!(result, expected);
        }
    }
}
