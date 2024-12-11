use std::fs;
fn parse_input(input_file_path: &str, data: &mut Vec<Vec<i32>>) {
    let input =
        fs::read_to_string(input_file_path).expect("Expected to parse string successfully!");
    let mut i = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            return;
        }
        // println!("line: {}", line);
        let items = line
            .split(' ')
            .map(|item| return item.trim().parse::<i32>().unwrap());
        let mut tmp: Vec<i32> = vec![];
        for (j, item) in items.enumerate() {
            tmp.insert(j, item);
        }
        data.insert(i, tmp);
        i += 1;
    }
}
fn is_safe(arr: &[i32]) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;
    for i in 0..arr.len() - 1 {
        let diff = arr[i] - arr[i + 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if arr[i] - arr[i + 1] > 0 {
            is_increasing = false;
        }
        if arr[i] - arr[i + 1] < 0 {
            is_decreasing = false;
        }
    }
    is_increasing || is_decreasing
}
fn is_also_safe(arr: &[i32]) -> bool {
    let safe = is_safe(arr);
    let mut also_safe = false;
    if !safe {
        for (i, _item) in arr.iter().enumerate() {
            let mut test = arr.to_vec();
            test.remove(i);
            also_safe = if is_safe(&test) {true} else {also_safe};
        }
    }
    safe || also_safe
}
pub fn part1(input_file_path: &str) {
    println!("Part 1, Day 2");
    let mut data = Vec::new();
    parse_input(input_file_path, &mut data);
    let mut c = 0;
    for line in data {
        if is_safe(&line) {
            c += 1;
        }
    }
    println!("Safe lines: {}", c);
}
pub fn part2(input_file_path: &str) {
    println!("Part 2, Day 2");
    let mut data = Vec::new();
    parse_input(input_file_path, &mut data);
    let mut c = 0;
    for line in data {
        if is_also_safe(&line) {
            c += 1;
        }
    }
    println!("Safe lines: {}", c);
}
