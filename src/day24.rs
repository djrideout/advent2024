use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Gate {
    input_0: String,
    input_1: String,
    op: String,
    output: String
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let mut wire_values: HashMap<String, i64> = HashMap::new();
    let mut gates_by_input: HashMap<String, Vec<Gate>> = HashMap::new();
    let mut gates_by_output: HashMap<String, Gate> = HashMap::new();
    let mut ready_wires: Vec<String> = vec![];
    let mut pending_wires: HashMap<String, String> = HashMap::new();
    let mut output_wires: Vec<String> = vec![];
    input.lines().for_each(|l| {
        if l.contains(": ") {
            let parts: Vec<&str> = l.split(": ").collect();
            wire_values.insert(parts[0].to_string(), parts[1].parse().unwrap());
        } else if l.contains(" -> ") {
            let parts: Vec<&str> = l.split(" -> ").collect();
            let input: Vec<&str> = parts[0].split(" ").collect();
            let gate = Gate {
                input_0: input[0].to_string(),
                input_1: input[2].to_string(),
                op: input[1].to_string(),
                output: parts[1].to_string()
            };
            if !gates_by_input.contains_key(&input[0].to_string()) {
                gates_by_input.insert(input[0].to_string(), vec![]);
            }
            if !gates_by_input.contains_key(&input[2].to_string()) {
                gates_by_input.insert(input[2].to_string(), vec![]);
            }
            gates_by_input.get_mut(input[0]).unwrap().push(gate.clone());
            gates_by_input.get_mut(input[2]).unwrap().push(gate.clone());
            gates_by_output.insert(parts[1].to_string(), gate.clone());
            if (input[0].starts_with("x") || input[0].starts_with("y")) &&
                (input[2].starts_with("x") || input[2].starts_with("y")) 
            {
                ready_wires.push(parts[1].to_string());
            }
            if parts[1].starts_with("z") {
                output_wires.push(parts[1].to_string());
            }
        }
    });
    while ready_wires.len() > 0 {
        let wire = ready_wires.pop().unwrap();
        let gate = gates_by_output.get(&wire).unwrap();
        let i0 = wire_values.get(&gate.input_0).unwrap();
        let i1 = wire_values.get(&gate.input_1).unwrap();
        let out = if gate.op == "AND" {
            i0 & i1
        } else if gate.op == "OR" {
            i0 | i1
        } else {
            i0 ^ i1
        };
        wire_values.insert(gate.output.clone(), out);
        if pending_wires.contains_key(&gate.output) {
            ready_wires.push(pending_wires.remove(&gate.output).unwrap());
        }
        if let Some(next_gates) = gates_by_input.get(&gate.output) {
            for gate in next_gates {
                let next_in_0 = wire_values.get(&gate.input_0);
                let next_in_1 = wire_values.get(&gate.input_1);
                if next_in_0.is_some() && next_in_1.is_some() {
                    ready_wires.push(gate.output.clone());
                } else {
                    if !next_in_0.is_some() {
                        pending_wires.insert(gate.input_0.clone(), gate.output.clone());
                    }
                    if !next_in_1.is_some() {
                        pending_wires.insert(gate.input_1.clone(), gate.output.clone());
                    }
                }
            }
        }
    }
    output_wires.sort();
    let mut out = 0;
    for i in 0 .. output_wires.len() {
        let wire = &output_wires[i];
        out |= wire_values.get(wire).unwrap() << i;
    }
    out
}

#[aoc(day24, part2)]
pub fn solve_part2(_: &str) -> String {
    // I mostly solved this by hand by comparing the bits of the true sum against the output of the adder
    // and comparing the structure of the gates with these images.
    // https://upload.wikimedia.org/wikipedia/commons/5/57/Fulladder.gif
    // https://upload.wikimedia.org/wikipedia/commons/8/85/RippleCarry2.gif
    let mut swaps = vec![
        "sps", "z11",
        "tst", "z05",
        "frt", "z23",
        "cgh", "pmd"
    ];
    swaps.sort();
    swaps.join(",")
}
