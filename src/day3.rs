use std::fs;
fn check_mul(instruction: &str) -> bool {
    if instruction.find(',').is_none() { return false; }
    let (first_part, second_part) = instruction.split_once(',').expect("Expected mul to split");
    let first_num = &first_part[4..];
    let second_num = &second_part[..second_part.len() - 1];
    for digit in first_num.chars() {
        if !digit.is_numeric() {
            return false;
        }
    }
    for digit in second_num.chars() {
        if !digit.is_numeric() {
            return false;
        }
    }
    true
}
fn eval_mul(instruction: &str) -> i32 {
    let (first_part, second_part) = instruction.split_once(',').expect("Expected mul to split");
    let first_num = str::parse::<i32>(&first_part[4..]).expect("Expect parse success");
    let second_num =
        str::parse::<i32>(&second_part[..second_part.len() - 1]).expect("Expect parse success");
    first_num * second_num
}
pub fn part1(input_file_path: &str) {
    let raw_str = fs::read_to_string(input_file_path).expect("File read error!!");
    let mut total: i32 = 0;
    raw_str.rmatch_indices("mul(").for_each(|(index, string)| {
        println!("{} --> {}", index, string);
        let tmp = raw_str.split_at(index).1;
        let end_index = tmp.find(')').expect("expected a closure");
        let trim_str = tmp.split_at(end_index + 1).0;
        println!("{}", trim_str);
        total += if check_mul(trim_str) {eval_mul(trim_str)} else {0};
    });
    println!("Eval result: {}", total);
}
