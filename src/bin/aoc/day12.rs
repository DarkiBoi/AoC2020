use std::str;

mod util;

fn main() {
    part_one();
    part_two();
}

pub fn part_one() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines {
        let char: char = line.as_bytes()[0] as char;
        let instruction = str::from_utf8(&line.as_bytes()[1..]).unwrap().parse::<u32>().unwrap();
        instructions.push(Instruction::new(char, instruction));
    }

    // Rotation 0 is east
    let mut rotation: i32 = 0;

    let mut north: i32 = 0;
    let mut east: i32 = 0;

    for instruction in instructions {
        let char = instruction.char;
        let instruction = instruction.instruction;

        match char {
            'N' => {
                north += instruction as i32;
            }
            'S' => {
                north -= instruction as i32;
            }
            'E' => {
                east += instruction as i32;
            }
            'W' => {
                east -= instruction as i32;
            }
            'L' => {
                rotation -= instruction as i32;
                rotation = (rotation + 360) % 360
            }
            'R' => {
                rotation += instruction as i32;
                rotation = (rotation + 360) % 360;
            }
            'F' => {
                if rotation >= 0 && rotation <= 89 {
                    east += instruction as i32;
                } else if rotation >= 90 && rotation <= 179 {
                    north -= instruction as i32;
                } else if rotation >= 180 && rotation <= 269 {
                    east -= instruction as i32;
                } else if rotation >= 270 && rotation <= 359 {
                    north += instruction as i32;
                }
            }
            _ => {}
        }
    }

    println!("Part One: {}", north.abs() + east.abs());
}


pub fn part_two() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines {
        let char: char = line.as_bytes()[0] as char;
        let instruction = str::from_utf8(&line.as_bytes()[1..]).unwrap().parse::<u32>().unwrap();
        instructions.push(Instruction::new(char, instruction));
    }

    let mut ship_north: i32 = 0;
    let mut ship_east: i32 = 0;

    // Both Relative
    let mut waypoint_north: i32 = 1;
    let mut waypoint_east: i32 = 10;

    for instruction in instructions {
        let char = instruction.char;
        let instruction = instruction.instruction;

        match char {
            'N' => {
                waypoint_north += instruction as i32;
            }
            'S' => {
                waypoint_north -= instruction as i32;
            }
            'E' => {
                waypoint_east += instruction as i32;
            }
            'W' => {
                waypoint_east -= instruction as i32;
            }
            'L' => {
                let degrees: i32 = ((instruction + 360) % 360) as i32;
                let mut x = waypoint_east;
                let mut y = waypoint_north;
                if degrees >= 0 && degrees <= 89 {
                    x = waypoint_north * -1;
                    y = waypoint_east;
                } else if degrees >= 90 && degrees <= 179 {
                    x = waypoint_north * -1;
                    y = waypoint_east;
                } else if degrees >= 180 && degrees <= 269 {
                    x = waypoint_east * -1;
                    y = waypoint_north * -1;
                } else if degrees >= 270 && degrees <= 359 {
                    x = waypoint_north;
                    y = waypoint_east * -1;
                }
                waypoint_east = x;
                waypoint_north = y;
            }
            'R' => {
                let degrees: i32 = ((instruction + 360) % 360) as i32;
                let mut x = waypoint_east;
                let mut y = waypoint_north;
                if degrees >= 0 && degrees <= 89 {
                    x = waypoint_east;
                    y = waypoint_north;
                } else if degrees >= 90 && degrees <= 179 {
                    x = waypoint_north;
                    y = waypoint_east * -1;
                } else if degrees >= 180 && degrees <= 269 {
                    x = waypoint_east * -1;
                    y = waypoint_north * -1;
                } else if degrees >= 270 && degrees <= 359 {
                    x = waypoint_north * -1;
                    y = waypoint_east;
                }
                waypoint_east = x;
                waypoint_north = y;
            }
            'F' => {
                for _i in 0..instruction {
                    ship_north += waypoint_north;
                    ship_east += waypoint_east;
                }
            }
            _ => {}
        }
    }

    println!("Part Two: {}", ship_north.abs() + ship_east.abs());
}


struct Instruction {
    char: char,
    instruction: u32,
}

impl Instruction {
    pub fn new(char: char, instruction: u32) -> Instruction {
        Instruction {
            char,
            instruction,
        }
    }
}