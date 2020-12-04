use std::collections::HashMap;
use regex::Regex;

mod util;

fn main() {
    part_one();
    part_two();
}

pub fn part_one() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut passports: Vec<HashMap<String, String>> = Vec::new();

    let mut current_passport: HashMap<String, String> = HashMap::new();

    let mut valid_passports: i32 = 0;

    for line in lines {
        let passport = &current_passport;

        if line == "" {
            passports.push(passport.to_owned());
            current_passport = HashMap::new();
            continue;
        }

        let pairs: Vec<&str> = line.split_ascii_whitespace().collect();
        for p in pairs {
            let pair: String = p.to_owned();
            let entry: Vec<&str> = pair.split(":").collect();
            let key: String = entry.get(0).unwrap().to_owned().to_string();
            let value: String = entry.get(1).unwrap().to_owned().to_string();
            &current_passport.insert(key, value);
        }
    }

    // Push last entry since this one isnt followed by a new line
    passports.push(current_passport);

    for passport in passports {
        if passport.contains_key("byr")
            && passport.contains_key("iyr")
            && passport.contains_key("eyr")
            && passport.contains_key("hgt")
            && passport.contains_key("hcl")
            && passport.contains_key("ecl")
            && passport.contains_key("pid") {
            valid_passports += 1;
        }
    }

    println!("Part One: {}", valid_passports)
}

pub fn part_two() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut passports: Vec<HashMap<String, String>> = Vec::new();

    let mut current_passport: HashMap<String, String> = HashMap::new();

    let mut valid_passports: i32 = 0;

    for line in lines {
        let passport = &current_passport;

        if line == "" {
            passports.push(passport.to_owned());
            current_passport = HashMap::new();
            continue;
        }

        let pairs: Vec<&str> = line.split_ascii_whitespace().collect();
        for p in pairs {
            let pair: String = p.to_owned();
            let entry: Vec<&str> = pair.split(":").collect();
            let key: String = entry.get(0).unwrap().to_owned().to_string();
            let value: String = entry.get(1).unwrap().to_owned().to_string();
            &current_passport.insert(key, value);
        }
    }

    // Push last entry since this one isn't followed by a new line
    passports.push(current_passport);

    for passport in passports {
        if valid_passport(passport) {
            if valid_passports == 122 {

            }
            valid_passports += 1;
        }
    }

    println!("Part Two: {}", valid_passports)
}

fn valid_passport(passport: HashMap<String, String>) -> bool {
    if !(passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")) {
        return false;
    }

    let birth_year_result = passport.get("byr").unwrap().parse::<i32>();
    if birth_year_result.is_ok() {
        let birth_year = birth_year_result.unwrap();
        if birth_year.to_string().len() != 4
            || birth_year < 1920 || birth_year > 2002 {
            return false;
        }
    } else {
        return false;
    }

    let issue_year_result = passport.get("iyr").unwrap().parse::<i32>();
    if issue_year_result.is_ok() {
        let issue_year = issue_year_result.unwrap();
        if issue_year.to_string().len() != 4
            || issue_year < 2010 || issue_year > 2020 {
            return false;
        }
    } else {
        return false;
    }

    let expiration_year_result = passport.get("eyr").unwrap().parse::<i32>();
    if expiration_year_result.is_ok() {
        let expiration_year = expiration_year_result.unwrap();
        if expiration_year.to_string().len() != 4
            || expiration_year < 2020 || expiration_year > 2030 {
            return false;
        }
    } else {
        return false;
    }

    let height = passport.get("hgt").unwrap();
    if height.contains("cm") {
        let height_number = height.replace("cm", "").parse::<i32>().unwrap();
        if height_number < 150 || height_number > 193 {
            return false;
        }
    } else if height.contains("in") {
        let height_number = height.replace("in", "").parse::<i32>().unwrap();
        if height_number < 59 || height_number > 76 {
            return false;
        }
    } else {
        return false;
    }

    let hair_color = passport.get("hcl").unwrap();
    let hair_color_regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    if !hair_color_regex.is_match(hair_color) {
        return false;
    }

    let eye_color = passport.get("ecl").unwrap();
    if eye_color != "amb"
        && eye_color != "blu"
        && eye_color != "brn"
        && eye_color != "gry"
        && eye_color != "grn"
        && eye_color != "hzl"
        && eye_color != "oth" {
        return false;
    }

    let passport_id = passport.get("pid").unwrap();
    let passport_id_regex = Regex::new(r"^[0-9]{9}$").unwrap();
    if !passport_id_regex.is_match(passport_id) {
        return false;
    }

    return true;
}