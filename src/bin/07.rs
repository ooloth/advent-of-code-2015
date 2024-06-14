advent_of_code::solution!(7);

use std::collections::HashMap;
use std::collections::HashSet;

fn get_signal_value(
    wire_or_signal: &str,
    wires_and_signals: &mut HashMap<String, u32>,
    instructions: &HashMap<String, Vec<String>>,
) -> u32 {
    match wire_or_signal.parse::<u32>() {
        // If parsing it as a u32 worked, it was a number:
        Ok(value) => value,

        // Otherwise, it was a wire identifier:
        Err(_) => {
            // If we don't have a key for this wire yet...
            if !wires_and_signals.contains_key(wire_or_signal) {
                // Calculate its value
                let value = match instructions.get(wire_or_signal) {
                    Some(instruction) => {
                        calculate_signal(instruction, wires_and_signals, instructions)
                    }
                    None => 0,
                };
                // and insert it
                wires_and_signals.insert(wire_or_signal.to_string(), value);
            }
            // Return this wire's value
            *wires_and_signals.get(wire_or_signal).unwrap()
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
        [signal] => get_signal_value(signal, wires_and_signals, instructions),

        // Two words: "NOT <wire>"
        [_, wire] => !get_signal_value(wire, wires_and_signals, instructions),

        // Three words: wire/signal AND/OR wire/signal, or wire/signal LSHIFT/RSHIFT number
        [left_operand, operator, right_operand] => {
            let left_signal = get_signal_value(left_operand, wires_and_signals, instructions);
            let right_signal = get_signal_value(right_operand, wires_and_signals, instructions);
            match operator.as_str() {
                "AND" => left_signal & right_signal,
                "OR" => left_signal | right_signal,
                "LSHIFT" => left_signal << right_signal,
                "RSHIFT" => left_signal >> right_signal,
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
    if visited.contains(wire) {
        return;
    };

    visited.insert(wire.to_string());

    if let Some(dependencies) = graph.get(wire) {
        for dependency in dependencies {
            visit(dependency, graph, visited, sorted_wires);
        }
    }

    sorted_wires.push(wire.to_string());
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

        // Iterate over each word in the instruction
        for &word in &instruction {
            // If the word is a wire identifier (because it isn't a number or an operator)...
            if word.parse::<u32>().is_err() && !operators.contains(&word) {
                // Add the wire to the target's list of dependencies
                dependency_graph
                    .entry(target.to_string())
                    .or_insert_with(Vec::new) // if not found, add target as a new key
                    .push(word.to_string());
            }
        }
    }

    let mut visited_wires: HashSet<String> = HashSet::new();
    let mut sorted_wires: Vec<String> = Vec::new();

    // Visit each wire in the dependency graph
    for wire in dependency_graph.keys() {
        visit(
            wire,
            &dependency_graph,
            &mut visited_wires,
            &mut sorted_wires,
        );
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

    Some(*wires_and_signals.get("a").unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    // Get the wire "a" signal from part one
    let a_signal_from_part_one = part_one(input);

    // Append a new instruction to the original setting wire "b" to that signal
    let new_input = format!("{}\n{} -> b", input, a_signal_from_part_one.unwrap());

    // Pass the new input through part_one again to get the new value of "a"
    part_one(&new_input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
