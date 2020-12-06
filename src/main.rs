mod tests;

use std::fs::File;
use std::io::{BufRead, BufReader};
use log::{info, debug};
use std::collections::HashSet;

fn read_input_data(file_name: &str) -> Vec<Vec<String>> {
    let f:File = File::open(file_name).unwrap();
    let f:BufReader<File> = BufReader::new(f);

    let mut answers:Vec<Vec<String>> = Vec::new();
    let mut current_group:Vec<String> = Vec::new();

    for line in f.lines() {
        let unwrapped_line = line.unwrap();
        if unwrapped_line == "" {
            answers.push(current_group);
            current_group = Vec::new();
            continue;
        }
        current_group.push(unwrapped_line);
    }
    answers.push(current_group);
    return answers;
}

fn solution_part_1(file_name: &str) -> i32 {
    let answers_by_group = read_input_data(file_name);
    let mut answers_set_by_group:Vec<HashSet<char>> = Vec::new();
    for answers_by_person in answers_by_group {
        let mut answers_set_of_group:HashSet<char> = HashSet::new();
        for answers_of_person in answers_by_person {
            for answer in answers_of_person.chars() {
                answers_set_of_group.insert(answer);
            }
        }
        answers_set_by_group.push(answers_set_of_group);
    }
    let mut yes_answers: i32 = 0;
    for answers_set in answers_set_by_group {
        debug!("{:?}", answers_set);
        yes_answers += answers_set.len() as i32;
    }
    return yes_answers;
}

fn answers_contain_code(code: char, answers: &[String]) -> bool {
    for answer in answers {
        if !(answer.contains(code)) {
            return false;
        }
    }
    return true;
}

fn solution_part_2(file_name: &str) -> i32 {
    let answers_by_group = read_input_data(file_name);
    let mut number_all_yes: i32 = 0;
    for answers_by_person in answers_by_group {
        for answer in answers_by_person[0].chars() {
            debug!("{}",answers_contain_code(answer, &answers_by_person[1..]));
            number_all_yes += answers_contain_code(answer, &answers_by_person[1..]) as i32;
        }
    }
    return number_all_yes
}

fn main() {
    env_logger::init();
    info!("{}", solution_part_1("inputData.txt"));
    info!("{}", solution_part_2("inputData.txt"));
}
