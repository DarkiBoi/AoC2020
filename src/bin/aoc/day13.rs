mod util;

fn main() {
    part_one();
    part_two();
}

pub fn part_one() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let arrival = lines.get(0).unwrap().parse::<i32>().unwrap();
    let mut bus_ids: Vec<i32> = Vec::new();

    for bus_id in lines.get(1).unwrap().split(",") {
        if bus_id == "x" {
            continue;
        }
        bus_ids.push(bus_id.parse::<i32>().unwrap())
    }

    let mut best_id = 0;
    let mut best_waiting_time = 99999;

    for bus_id in &bus_ids {
        if (bus_id * (arrival / bus_id + 1)) - arrival < best_waiting_time {
            best_id = *bus_id;
            best_waiting_time = (bus_id * (arrival / bus_id + 1)) - arrival;
        }
    }

    println!("Part One: {}", best_id * best_waiting_time);
}

pub fn part_two() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut bus_ids: Vec<i32> = Vec::new();

    for bus_id in lines.get(1).unwrap().split(",") {
        if bus_id == "x" {
            bus_ids.push(0);
            continue;
        }
        bus_ids.push(bus_id.parse::<i32>().unwrap())
    }


    let mut timestamp = 0;
    let mut pattern_product = 1;

    for mut i in 0u64..bus_ids.len() as u64 {
        let id = *&bus_ids[i as usize] as u64;

        // 0 signalizes "x" in input set, nothing to check here
        if id == 0 {
            continue;
        }
        
        i = id - i % id;

        // Handling for first iteration
        if i == id {
            i = 0;
        }

        // While it hasn't found the next bus, increment the timestamp by the pattern product, since the prior buses only appear at this interval
        while timestamp % id != i {
            timestamp += pattern_product;
        }

        // Once we have found one bus we know that this bus will only repeat at every n. For two busses it will be n1 * n2 and so on.
        pattern_product *= id;
    }



    println!("Part Two: {}", timestamp);
}