mod util;

fn main() {
    part_one();
    part_two();
}

pub fn part_one() {

    let lines: Vec<i64> = util::get_input_lines().map(|s| s.parse::<i64>().unwrap()).collect();

    let mut broken_number: i64 = 0;

    for i in 25..lines.len() {

        let preamble = &lines[i - 25..i + 1];
        let number = &lines[i];

        let mut valid: bool = false;
        for p in 0..preamble.len() {
            let num_1 = preamble[p];
            for k in 0..preamble.len() {
                let num_2 = preamble[k];
                if p == k {
                    continue
                }
                if num_1 + num_2 == *number {
                    valid = true;
                    break;
                }
            }
        }

        if !valid {
            broken_number = *number;
        }

    }

    println!("Part One: {}", broken_number);

}

pub fn part_two() {

    let lines: Vec<i64> = util::get_input_lines().map(|s| s.parse::<i64>().unwrap()).collect();

    let mut broken_number: i64 = 0;

    for i in 25..lines.len() {

        let preamble = &lines[i - 25..i + 1];
        let number = &lines[i];

        let mut valid: bool = false;
        for p in 0..preamble.len() {
            let num_1 = preamble[p];
            for k in 0..preamble.len() {
                let num_2 = preamble[k];
                if p == k {
                    continue
                }
                if num_1 + num_2 == *number {
                    valid = true;
                    break;
                }
            }
        }

        if !valid {
            broken_number = *number;
        }

    }

    let mut exploit_number: i64 = 0;

    for i in 0..lines.len() {

        let test_num = lines[i];

        let mut sum = test_num;

        let mut range: Vec<i64> = Vec::new();

        range.push(test_num);

        for p in i + 1..lines.len() {

            let sum_num = lines[p];

            range.push(sum_num);

            sum += sum_num;

            if sum == broken_number {
                exploit_number = range.iter().max().unwrap() + range.iter().min().unwrap();
            } else if sum > broken_number {
                break
            }

        }

    }

    println!("Part Two: {}", exploit_number);

}