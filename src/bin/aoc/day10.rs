mod util;

fn main() {
    part_one();
}

pub fn part_one() {

    let mut lines: Vec<i32> = util::get_input_lines().map(|s| s.parse::<i32>().unwrap()).collect();

    let max = lines.iter().max().unwrap() + 3;

    lines.push(max);

    let mut current_adapter = 0;

    let mut one_diff_count = 0;
    let mut three_diff_count = 0;

    while current_adapter < max {

        let mut candidate = 0;
        let mut candidate_diff = 5;

        for num in &lines {
            let diff = (num - current_adapter).abs();
            if diff > 3 {
                continue
            }
            if *num > current_adapter && candidate_diff > diff {
                candidate = *num;
                candidate_diff = diff;
            }
        }

        if candidate_diff == 1 {
            one_diff_count += 1;
        } else if candidate_diff == 3 {
            three_diff_count += 1;
        }

        current_adapter = candidate;
    }

    println!("Part One: {}", one_diff_count * three_diff_count);

}