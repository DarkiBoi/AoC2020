mod util;

fn main() {
    part_one();
    part_two()
}

pub fn part_one() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut valid_count: i32 = 0;

    for line in lines {
        let mut min: i32 = 0;
        let mut max: i32 = 0;
        let mut char: char = 0 as char;
        let mut password: &str = "";

        let splits: Vec<&str> = line.split_ascii_whitespace().collect();

        for split in splits {
            if split.contains("-") {
                let min_max: Vec<&str> = split.split("-").collect();
                min = *&min_max[0].parse::<i32>().unwrap();
                max = *&min_max[1].parse::<i32>().unwrap();
            } else if split.contains(":") {
                char = split.replace(":", "").parse::<char>().unwrap();
            } else {
                password = split;
            }
        }

        let mut char_count: i32 = 0;

        for c in password.chars() {
            if c == char {
                char_count += 1;
            }
        }

        if char_count < min || char_count > max {
            continue;
        }

        valid_count += 1;
    }

    println!("Part One: Found {} valid passwords!", valid_count)
}

pub fn part_two() {

    let lines: Vec<String> = util::get_input_lines().collect();

    let mut valid_count: i32 = 0;

    for line in lines {
        let mut first_position: i32 = 0;
        let mut second_position: i32 = 0;
        let mut char: char = 0 as char;
        let mut password: &str = "";

        let splits: Vec<&str> = line.split_ascii_whitespace().collect();

        for split in splits {
            if split.contains("-") {
                let positions: Vec<&str> = split.split("-").collect();
                first_position = *&positions[0].parse::<i32>().unwrap();
                second_position = *&positions[1].parse::<i32>().unwrap();
            } else if split.contains(":") {
                char = split.replace(":", "").parse::<char>().unwrap();
            } else {
                password = split;
            }
        }

        let mut occurrences: i32 = 0;

        for i in 0..password.len() {
            let c = password.as_bytes()[i] as char;
            if i + 1 == first_position as usize || i + 1 == second_position as usize {
                if c == char {
                    occurrences += 1;
                }
            }
        }

        if occurrences == 1 {
            valid_count += 1;
        }
    }

    println!("Part Two: Found {} valid passwords!", valid_count)

}