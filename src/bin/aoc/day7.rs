use std::collections::HashMap;

mod util;

fn main() {
    part_one();
    part_two();
}


pub fn part_one() {
    let mut bags: HashMap<String, HashMap<String, i32>> = HashMap::new();

    let lines: Vec<String> = util::get_input_lines().collect();

    for line in lines {
        let bag_split: Vec<&str> = line.split("contain").collect();

        let outer_bag = &bag_split[0].replace("bags", "").replace(" ", "");

        let inner_bags: Vec<&str> = bag_split[1].split(",").collect();

        let mut sub_bags: HashMap<String, i32> = HashMap::new();

        for i in 0..inner_bags.len() {
            let inner_bag = inner_bags[i].replace("bags", "").replace("bag", "").replace(".", "");

            let space_split: Vec<&str> = inner_bag.split_ascii_whitespace().collect();

            if space_split[0] == "no" {
                continue;
            }

            let num = space_split[0].parse::<i32>().unwrap();
            let color = space_split[1].to_owned() + &space_split[2].to_owned();

            sub_bags.insert(color, num);
        }

        bags.insert(outer_bag.to_owned().parse().unwrap(), sub_bags);
    }

    let mut count = 0;

    for outer_bag in bags.keys() {
        if outer_bag != "shinygold" &&
            contains_gold(outer_bag, &bags) {
            count += 1;
        }
    }

    println!("Part One: {}", count);
}

pub fn part_two() {
    let mut bags: HashMap<String, HashMap<String, i32>> = HashMap::new();

    let lines: Vec<String> = util::get_input_lines().collect();

    for line in lines {
        let bag_split: Vec<&str> = line.split("contain").collect();

        let outer_bag = &bag_split[0].replace("bags", "").replace(" ", "");

        let inner_bags: Vec<&str> = bag_split[1].split(",").collect();

        let mut sub_bags: HashMap<String, i32> = HashMap::new();

        for i in 0..inner_bags.len() {
            let inner_bag = inner_bags[i].replace("bags", "").replace("bag", "").replace(".", "");

            let space_split: Vec<&str> = inner_bag.split_ascii_whitespace().collect();

            if space_split[0] == "no" {
                continue;
            }

            let num = space_split[0].parse::<i32>().unwrap();
            let color = space_split[1].to_owned() + &space_split[2].to_owned();

            sub_bags.insert(color, num);
        }

        bags.insert(outer_bag.to_owned().parse().unwrap(), sub_bags);
    }

    let total_bag_count = count_inner_bags(&"shinygold".to_string(), &bags);

    println!("Part Two: {}", total_bag_count);
}

fn contains_gold(bag: &String, bags: &HashMap<String, HashMap<String, i32>>) -> bool {
    if bag == "shinygold" {
        return true;
    } else {
        for inner_bag in bags.get(bag).unwrap().keys() {
            if contains_gold(inner_bag, &bags) {
                return true;
            }
        }
    }
    return false;
}

fn count_inner_bags(bag: &String, bags: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut count = 0;
    for inner_bag in bags.get(bag).unwrap().keys() {
        count += bags.get(bag).unwrap().get(inner_bag).unwrap() + bags.get(bag).unwrap().get(inner_bag).unwrap() * count_inner_bags(inner_bag, bags);
    }
    return count;
}