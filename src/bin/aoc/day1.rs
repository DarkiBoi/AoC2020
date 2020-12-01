mod util;

fn main() {
    part_one();
    part_two();
}

pub fn part_one() {
    let lines: Vec<i32> = util::get_input_lines().map(|s| s.parse::<i32>().unwrap()).collect();

    for i in 0..lines.len() {
        let first = lines[i];
        for p in  0..lines.len() {
            if p == i {
                continue
            }
            let second = lines[p];
            if(first + second) == 2020 {
                println!("Part One: {}", first * second);
                return;
            }
        }
    }
}

pub fn part_two() {
    let lines: Vec<i32> = util::get_input_lines().map(|s| s.parse::<i32>().unwrap()).collect();

    for i in 0..lines.len() {
        let first = lines[i];
        for p in  0..lines.len() {
            if p == i {
                continue
            }
            let second = lines[p];
            for q in 0..lines.len() {
                if q == p || q == i {
                    continue
                }
                let third = lines[q];
                if(first + second + third) == 2020 {
                    println!("Part Two: {}", first * second * third);
                    return;
                }
            }

        }
    }

}
