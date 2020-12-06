mod util;

fn main() {
    part_one();
    part_two();
}

pub fn part_one() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut groups: Vec<String> = Vec::new();

    let mut group: String = "".to_string();

    for line in lines {
        if line == "" {
            groups.push(group.to_owned());
            group = "".to_string();
        }
        group += &*line;
    }

    groups.push(group.to_owned());

    let mut questions_sum = 0;

    for group in groups {
        let mut answered_questions: Vec<char> = Vec::new();

        for char in group.chars() {
            if !answered_questions.contains(&char) {
                answered_questions.push(char);
            }
        }

        questions_sum += answered_questions.len();
    }

    println!("Part One: {}", questions_sum);
}

pub fn part_two() {
    let lines: Vec<String> = util::get_input_lines().collect();

    let mut groups: Vec<Vec<String>> = Vec::new();

    let mut group_members: Vec<String> = Vec::new();

    for line in lines {
        if line == "" {
            groups.push(group_members);
            group_members = Vec::new();
            continue;
        }
        group_members.push(line);
    }

    groups.push(group_members);

    let mut questions_sum = 0;

    for group in groups {
        let mut answered_questions: Vec<char> = Vec::new();

        let mut group_size = 0;

        for group_member in group {
            group_size += 1;
            for char in group_member.chars() {
                answered_questions.push(char);
            }
        }

        let mut handled_chars: Vec<&char> = Vec::new();

        for question in &answered_questions {
            if handled_chars.contains(&question) {
                continue
            }
            if count_chars(&answered_questions, question) == group_size {
                questions_sum += 1;
                handled_chars.push(question);
            }
        }
    }

    println!("Part Two: {}", questions_sum);
}

fn count_chars(chars: &Vec<char>, char: &char) -> i32 {
    let mut count = 0;
    for c in chars {
        if c == char {
            count += 1;
        }
    }
    return count;
}