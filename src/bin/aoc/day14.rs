use std::fmt::Debug;
use std::str;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;

mod util;

fn main() {
    part_one();
    part_two();
}

pub fn part_one() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut mask_and_mem: LinkedHashMap<Vec<u8>, Vec<MemoryAddress>> = LinkedHashMap::new();

    let mut current_mask: Vec<u8> = Vec::new();
    let mut current_mem: Vec<MemoryAddress> = Vec::new();

    for line in lines {
        let split: Vec<&str> = line.split("=").collect();
        if split[0].replace(" ", "") == "mask" {
            if !current_mask.is_empty() {
                mask_and_mem.insert(current_mask, (*current_mem).to_vec());
                current_mask = Vec::new();
                current_mem = Vec::new();
            }
            for bit in split[1].chars() {
                match bit {
                    '0' => {
                        current_mask.push(0);
                    }
                    '1' => {
                        current_mask.push(1);
                    }
                    'X' => {
                        current_mask.push(2);
                    }
                    _ => {}
                }
            }
        } else if split[0].starts_with("mem") {
            let address = split[0].replace("mem[", "").replace("]", "").replace(" ", "").replace(" ", "").parse::<u64>().unwrap();
            let value = split[1].replace(" ", "").parse::<u64>().unwrap();
            current_mem.push(MemoryAddress::new(address, value));
        }
    }

    mask_and_mem.insert(current_mask, (*current_mem).to_vec());

    let mut memory: HashMap<u64, u64> = HashMap::new();

    for entry in &mask_and_mem {
        let mask = entry.0;

        for mem_addr in entry.1 {
            let bits: Vec<char> = format!("{:b}", mem_addr.value).chars().collect();
            let mut value: Vec<u8> = vec!(0 as u8; 36);
            for i in 0..value.len() {
                if i >= value.len() - bits.len() {
                    match bits[i - (value.len() - bits.len())] {
                        '1' => value[i] = 1 as u8,
                        '0' => value[i] = 0 as u8,
                        _ => {}
                    }
                }
            }
            let mut output: [u8; 36] = [0; 36];

            for (index, bit) in value.iter().enumerate() {
                if *mask.get(index).unwrap() != 2 as u8 {
                    output[index] = *mask.get(index).unwrap();
                } else {
                    output[index] = *bit;
                }
            }

            let mut char_array: Vec<char> = Vec::new();

            for byte in output.iter() {
                match byte {
                    0u8 => {
                        char_array.push('0');
                    }
                    1u8 => {
                        char_array.push('1');
                    }
                    _ => {}
                }
            }

            let output_string: String = char_array.iter().collect();

            let result_value = u64::from_str_radix(output_string.as_str(), 2).unwrap();

            memory.insert(mem_addr.address, result_value);
        }
    }

    let mut sum: u64 = 0;

    for entry in memory {
        sum += entry.1;
    }

    println!("Part One: {}", sum);
}

pub fn part_two() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut mask_and_mem: LinkedHashMap<Vec<u8>, Vec<MemoryAddress>> = LinkedHashMap::new();

    let mut current_mask: Vec<u8> = Vec::new();
    let mut current_mem: Vec<MemoryAddress> = Vec::new();

    for line in lines {
        let split: Vec<&str> = line.split("=").collect();
        if split[0].replace(" ", "") == "mask" {
            if !current_mask.is_empty() {
                mask_and_mem.insert(current_mask, (*current_mem).to_vec());
                current_mask = Vec::new();
                current_mem = Vec::new();
            }
            for bit in split[1].chars() {
                match bit {
                    '0' => {
                        current_mask.push(0);
                    }
                    '1' => {
                        current_mask.push(1);
                    }
                    'X' => {
                        current_mask.push(2);
                    }
                    _ => {}
                }
            }
        } else if split[0].starts_with("mem") {
            let address = split[0].replace("mem[", "").replace("]", "").replace(" ", "").replace(" ", "").parse::<u64>().unwrap();
            let value = split[1].replace(" ", "").parse::<u64>().unwrap();
            current_mem.push(MemoryAddress::new(address, value));
        }
    }

    mask_and_mem.insert(current_mask, (*current_mem).to_vec());

    let mut memory: HashMap<u64, u64> = HashMap::new();

    for entry in &mask_and_mem {
        let mask = entry.0;

        for mem_addr in entry.1 {
            let bits: Vec<char> = format!("{:b}", mem_addr.value).chars().collect();
            let mut value: Vec<u8> = vec!(0 as u8; 36);
            for i in 0..value.len() {
                if i >= value.len() - bits.len() {
                    match bits[i - (value.len() - bits.len())] {
                        '1' => value[i] = 1 as u8,
                        '0' => value[i] = 0 as u8,
                        _ => {}
                    }
                }
            }
            let mut output: [u8; 36] = [0; 36];

            for (index, bit) in value.iter().enumerate() {
                if *mask.get(index).unwrap() != 2 as u8 {
                    output[index] = *mask.get(index).unwrap();
                } else {
                    output[index] = *bit;
                }
            }


            let mut char_array: Vec<char> = Vec::new();

            for byte in output.iter() {
                match byte {
                    0u8 => {
                        char_array.push('0');
                    }
                    1u8 => {
                        char_array.push('1');
                    }
                    _ => {}
                }
            }

            let output_string: String = char_array.iter().collect();

            let result_value = u64::from_str_radix(output_string.as_str(), 2).unwrap();

            // Memory Address Mask Bit Shit

            let address_bits: Vec<char> = format!("{:b}", mem_addr.address).chars().collect();
            let mut address_value: String = String::with_capacity(36);
            for i in 0..address_value.capacity() {
                if i >= address_value.capacity() - address_bits.len() {
                    address_value.push(address_bits[i - (address_value.capacity() - address_bits.len())]);
                } else {
                    address_value.push('0');
                }
            }

            println!("{}", address_value);

            let mut output_address: String = String::with_capacity(36);

            let mut floating_count: i32 = 0;

            for (index, bit) in address_value.chars().into_iter().enumerate() {
                match *mask.get(index).unwrap() {
                    0 => {
                        output_address.push(bit);
                    }
                    1 => {
                        output_address.push('1');
                    }
                    2 => {
                        output_address.push('0');
                        floating_count += 1;
                    }
                    _ => {}
                }
            }

            let mut masked_address: u64 = u64::from_str_radix(output_address.as_str(), 2).unwrap();

            let mut addresses: Vec<u64> = Vec::new();

            let mut floating_decimal: u64 = 0;

            for i in 0..floating_count.pow(2) {
                let address = masked_address | floating_decimal;
                addresses.push(address);
                println!("{}", address);
                println!("{:b}", address);
                floating_decimal += 0b1;
            }

            for address in addresses {
                memory.insert(address, result_value);
            }
        }
    }

    let mut sum: u64 = 0;

    for entry in memory {
        sum += entry.1;
    }

    println!("Part Two (Doesnt work): {}", sum);
}

#[derive(Debug)]
struct MemoryAddress {
    address: u64,
    value: u64,
}

impl MemoryAddress {
    pub fn new(address: u64, value: u64) -> MemoryAddress {
        MemoryAddress {
            address,
            value,
        }
    }
}


impl Clone for MemoryAddress {
    fn clone(&self) -> Self {
        MemoryAddress {
            address: self.address,
            value: self.value,
        }
    }
}