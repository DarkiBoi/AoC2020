mod util;

fn main() {
    part_one();
    part_two();
}

pub fn part_one() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut grid = vec!(vec![".".parse::<char>().unwrap(); lines[0].len()]; lines.len());

    for i in 0..lines.len() {
        let line = &lines[i];
        for c in 0..line.as_bytes().len() {
            let char = line.as_bytes()[c] as char;
            grid[i][c] = char;
        }
    }

    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut trees_encountered: i32 = 0;


    while y < grid.len() {

        let char = grid[y][x % (grid[y].len())];

        if char == "#".parse::<char>().unwrap() {
            trees_encountered += 1;
        }

        x += 3;
        y += 1;
    }

    println!("Part One: {:?}", trees_encountered);

}

pub fn part_two() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut grid = vec!(vec![".".parse::<char>().unwrap(); lines[0].len()]; lines.len());

    for i in 0..lines.len() {
        let line = &lines[i];
        for c in 0..line.as_bytes().len() {
            let char = line.as_bytes()[c] as char;
            grid[i][c] = char;
        }
    }

    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut trees_encountered: usize = 0;

    let mut final_result: usize = 1;

    let mut offset_x = 1;
    let mut offset_y = 1;

    while y < grid.len() {

        if offset_x > 9 {
            break;
        }

        let char = grid[y][x % (grid[y].len())];

        if char == "#".parse::<char>().unwrap() {
            trees_encountered += 1;
        }

        if y == grid.len() - 1 {
            y = 0;
            x = 0;
            offset_x += 2;
            offset_y = (((offset_x / 8) as f32).floor()) as usize + 1;
            final_result *= trees_encountered;
            trees_encountered = 0;
        }

        x += offset_x % 8;
        y += offset_y;
    }

    println!("Part Two: {:?}", final_result);

}