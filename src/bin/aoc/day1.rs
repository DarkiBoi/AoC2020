mod util;

fn main() {
    let lines: Vec<i32> = util::get_input_lines().map(|s| s.parse::<i32>().unwrap()).collect();

    for i in 0..lines.len() {
        let first = lines[i];
        for p in  0..lines.len() {
            if p == i {
                continue
            }
            let second = lines[p];
            if(first + second) == 2020 {
                println!("Result found! {}", first * second);
                return;
            }
        }
    }
}
