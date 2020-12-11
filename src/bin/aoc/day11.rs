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

    let mut current_seats = grid;

    loop {
        let (new_seats, changed) = run_round((&current_seats).to_owned());

        if changed > 0 {
            current_seats = new_seats;
        } else {
            break;
        }
    }

    println!("Part One: {:?}", count_occupied(current_seats));
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

    let mut current_seats = grid;
    // This shit takes literally 3 minutes to compute, and idk how to optimize it xD

    loop {
        let (new_seats, changed) = run_round_with_vision((&current_seats).to_owned());

        if changed > 0 {
            current_seats = new_seats;
        } else {
            break;
        }
    }


    println!("Part Two: {:?}", count_occupied(current_seats));
}

fn run_round(input_seats: Vec<Vec<char>>) -> (Vec<Vec<char>>, u32) {
    let mut seats = input_seats;

    let mut changed = 0u32;

    let mut seats_to_be_changed: Vec<Change> = Vec::new();

    for y in 0i64..seats.len() as i64 {
        for x in 0i64..seats[y as usize].len() as i64 {
            let seat = &seats[y as usize][x as usize];
            if *seat == ".".parse::<char>().unwrap() {
                continue;
            } else if *seat == "L".parse::<char>().unwrap() {
                let mut next_to_occupied: bool = false;
                for i in -1i64..=1 {
                    for p in -1i64..=1 {
                        if i == 0 && p == 0 {
                            continue;
                        }

                        if y + i < 0 || x + p < 0 {
                            continue;
                        }

                        if y + i >= seats.len() as i64 || x + p >= seats[(y + i) as usize].len() as i64 {
                            continue;
                        }

                        let seat_check = &seats[(y + i) as usize][(x + p) as usize];
                        if *seat_check == "#".parse::<char>().unwrap() {
                            next_to_occupied = true;
                        }
                    }
                }
                if !next_to_occupied {
                    let change = Change::new(x, y, "#".parse::<char>().unwrap());
                    seats_to_be_changed.push(change);
                    changed += 1;
                }
            } else if *seat == "#".parse::<char>().unwrap() {
                let mut occupied_adjacents = 0;
                for i in -1i64..=1 {
                    for p in -1i64..=1 {
                        if i == 0 && p == 0 {
                            continue;
                        }

                        if y + i < 0 || x + p < 0 {
                            continue;
                        }

                        if y + i >= seats.len() as i64 || x + p >= seats[(y + i) as usize].len() as i64 {
                            continue;
                        }

                        let seat_check = &seats[(y + i) as usize][(x + p) as usize];
                        if *seat_check == "#".parse::<char>().unwrap() {
                            occupied_adjacents += 1;
                        }
                    }
                }
                if occupied_adjacents >= 4 {
                    let change = Change::new(x, y, "L".parse::<char>().unwrap());
                    seats_to_be_changed.push(change);
                    changed += 1;
                }
            }
        }
    }

    for change in seats_to_be_changed {
        seats[change.coordinate.y as usize][change.coordinate.x as usize] = change.char;
    }

    return (seats, changed);
}

fn run_round_with_vision(input_seats: Vec<Vec<char>>) -> (Vec<Vec<char>>, u32) {
    let mut seats = input_seats;

    let mut changed = 0u32;

    let mut seats_to_be_changed: Vec<Change> = Vec::new();

    for y in 0i64..seats.len() as i64 {
        for x in 0i64..seats[y as usize].len() as i64 {
            let seat = &seats[y as usize][x as usize];
            if *seat == ".".parse::<char>().unwrap() {
                continue;
            } else if *seat == "L".parse::<char>().unwrap() {
                let mut next_to_occupied: bool = false;
                for i in -1i64..=1 {
                    for p in -1i64..=1 {
                        if i == 0 && p == 0 {
                            continue;
                        }
                        let seat_check = see_direction(Coordinate::new(x, y), (&seats).to_owned(), p, i);
                        if seat_check == "#".parse::<char>().unwrap() {
                            next_to_occupied = true;
                        }
                    }
                }
                if !next_to_occupied {
                    let change = Change::new(x, y, "#".parse::<char>().unwrap());
                    seats_to_be_changed.push(change);
                    changed += 1;
                }
            } else if *seat == "#".parse::<char>().unwrap() {
                let mut occupied_adjacents = 0;
                for i in -1i64..=1 {
                    for p in -1i64..=1 {
                        if i == 0 && p == 0 {
                            continue;
                        }
                        let seat_check = see_direction(Coordinate::new(x, y), (&seats).to_owned(), p, i);
                        if seat_check == "#".parse::<char>().unwrap() {
                            occupied_adjacents += 1;
                        }
                    }
                }
                if occupied_adjacents >= 5 {
                    let change = Change::new(x, y, "L".parse::<char>().unwrap());
                    seats_to_be_changed.push(change);
                    changed += 1;
                }
            }
        }
    }

    for change in seats_to_be_changed {
        seats[change.coordinate.y as usize][change.coordinate.x as usize] = change.char;
    }

    return (seats, changed);
}

fn see_direction(seat: Coordinate, seats: Vec<Vec<char>>, x_offset: i64, y_offset: i64) -> char {
    let mut spotted_char = ".".parse::<char>().unwrap();

    let mut current_vision: Coordinate = seat;

    while current_vision.x + x_offset >= 0 && current_vision.x + x_offset < seats[current_vision.y as usize].len() as i64 && current_vision.y + y_offset >= 0 && current_vision.y + y_offset < seats.len() as i64 {
        current_vision.x += x_offset;
        current_vision.y += y_offset;
        let char = seats[current_vision.y as usize][current_vision.x as usize];
        if char != spotted_char {
            spotted_char = char;
            break;
        }
    }

    return spotted_char;
}

fn count_occupied(seats: Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for y in 0..seats.len() {
        for x in 0..seats[y].len() {
            let seat = &seats[y][x];
            if *seat == "#".parse::<char>().unwrap() {
                count += 1;
            }
        }
    }
    return count;
}

pub struct Change {
    coordinate: Coordinate,
    char: char,
}

pub struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Coordinate {
        Coordinate {
            x,
            y,
        }
    }
}

impl Change {
    pub fn new(x: i64, y: i64, char: char) -> Change {
        let coordinate = Coordinate::new(x, y);
        Change {
            coordinate,
            char,
        }
    }
}