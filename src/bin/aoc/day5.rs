use std::cmp::max;

mod util;

fn main() {
    part_one();
    part_two();
}

pub fn part_one() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut boarding_passes_ids: Vec<i32> = Vec::new();

    for line in lines {
        let mut rows: Vec<i32> = vec!(0; 128);

        for i in 0..rows.len() {
            rows[i] = (i + 1) as i32;
        }

        let mut cols: Vec<i32> = vec!(0; 8);

        for i in 0..cols.len() {
            cols[i] = (i + 1) as i32;
        }

        let row_spec = &line[0..7];
        let col_spec = &line[7..line.len()];

        for i in 0..row_spec.as_bytes().len() {
            let char = row_spec.as_bytes()[i] as char;
            if char == "F".parse::<char>().unwrap() {
                rows = rows[..rows.len() / 2].to_owned();
            } else if char == "B".parse::<char>().unwrap() {
                rows = rows[rows.len() / 2..].to_owned()
            }
        }

        for i in 0..col_spec.as_bytes().len() {
            let char = col_spec.as_bytes()[i] as char;
            if char == "L".parse::<char>().unwrap() {
                cols = cols[..cols.len() / 2].to_owned();
            } else if char == "R".parse::<char>().unwrap() {
                cols = cols[cols.len() / 2..].to_owned()
            }
        }

        let row = *rows.get(0).unwrap() - 1;
        let col = *cols.get(0).unwrap() - 1;
        boarding_passes_ids.push(row * 8 + col);
    }

    let mut highest_id = 0;
    for id in boarding_passes_ids {
        highest_id = max(highest_id, id);
    }

    println!("Part One: {}", highest_id);
}

pub fn part_two() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut boarding_passes_ids: Vec<i32> = Vec::new();

    for line in lines {
        let mut rows: Vec<i32> = vec!(0; 128);

        for i in 0..rows.len() {
            rows[i] = (i + 1) as i32;
        }

        let mut cols: Vec<i32> = vec!(0; 8);

        for i in 0..cols.len() {
            cols[i] = (i + 1) as i32;
        }

        let row_spec = &line[0..7];
        let col_spec = &line[7..line.len()];

        for i in 0..row_spec.as_bytes().len() {
            let char = row_spec.as_bytes()[i] as char;
            if char == "F".parse::<char>().unwrap() {
                rows = rows[..rows.len() / 2].to_owned();
            } else if char == "B".parse::<char>().unwrap() {
                rows = rows[rows.len() / 2..].to_owned()
            }
        }

        for i in 0..col_spec.as_bytes().len() {
            let char = col_spec.as_bytes()[i] as char;
            if char == "L".parse::<char>().unwrap() {
                cols = cols[..cols.len() / 2].to_owned();
            } else if char == "R".parse::<char>().unwrap() {
                cols = cols[cols.len() / 2..].to_owned()
            }
        }

        let row = *rows.get(0).unwrap() - 1;
        let col = *cols.get(0).unwrap() - 1;
        boarding_passes_ids.push(row * 8 + col);
    }

    let mut seat_id = 0;

    for i in 1..boarding_passes_ids.len() - 1 {
        let id = boarding_passes_ids[i];
        if !boarding_passes_ids.contains(&(id + 1))
            && boarding_passes_ids.contains(&(id + 2)) {
            seat_id = id + 1;
            break;
        }
    }

    println!("Part Two: {}", seat_id);
}