use std::collections::HashMap;

mod util;

fn main() {
    part_one();
    part_two();
}

pub fn part_one() {

    let lines: Vec<String> = util::get_input_lines().collect();

    let mut instructions: Vec<HashMap<String, i32>> = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut hashmap = HashMap::new();
        let arg = split.get(1).unwrap().replace("+", "").parse::<i32>().unwrap();
        hashmap.insert(split.get(0).unwrap().to_string(), arg);
        instructions.push(hashmap);
    }

    println!("Part One: {}", run_instructions(instructions, true))

}

pub fn part_two() {

    let lines: Vec<String> = util::get_input_lines().collect();

    let mut instructions: Vec<HashMap<String, i32>> = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut hashmap = HashMap::new();
        let arg = split.get(1).unwrap().replace("+", "").parse::<i32>().unwrap();
        hashmap.insert(split.get(0).unwrap().to_string(), arg);
        instructions.push(hashmap);
    }

    let mut final_acc = 0;

    for i in 0..instructions.len() {

        let instruction = &instructions[i];
        let opcodes: Vec<&String> = instruction.keys().collect();
        let opcode = opcodes[0];
        let arg = instruction.get(opcode).unwrap();

        let mut instruction_copy = instructions.clone();

        let mut new_hash_map = HashMap::new();
        if opcode == "jmp" {
            new_hash_map.insert("nop".to_string(), *arg);
            instruction_copy[i] = new_hash_map;
        } else if opcode == "nop" {
            new_hash_map.insert("jmp".to_string(), *arg);
            instruction_copy[i] = new_hash_map;
        } else {
            continue
        }

        let run_result = run_instructions(instruction_copy, false);

        if run_result != 0 {
            final_acc = run_result;
            break;
        }
    }

    println!("Part Two: {}", final_acc);

}

fn run_instructions(instructions: Vec<HashMap<String, i32>>, fuck_infinite: bool) -> i32 {

    let mut index: i32 = 0;
    let mut accumulator = 0;
    let mut instructions_handled: Vec<i32> = Vec::new();

    while index < instructions.len() as i32 {

        if instructions_handled.contains(&(index as i32)) {
            if !fuck_infinite {
                accumulator = 0;
            }
            break;
        }

        let instruction: &HashMap<String, i32> = &instructions[index as usize];

        let opcodes: Vec<&String> = instruction.keys().collect();
        let opcode = opcodes[0];
        let arg = instruction.get(opcode).unwrap();

        instructions_handled.push(index as i32);

        if opcode == "nop" {
            index += 1;
        } else if opcode == "acc" {
            accumulator += arg;
            index += 1;
        } else if opcode == "jmp" {
            index += *arg;
        }
    }

    return accumulator;
}