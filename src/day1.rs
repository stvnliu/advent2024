use std::fs;
fn parse_input(input_string: &str, column1: &mut Vec<i32>, column2: &mut Vec<i32>) {
    let mut counter = 0;
    input_string.split('\n').for_each(|line| {
        if line.is_empty() {
            return;
        }
        let values = line
            .split_once(' ')
            .expect("Expected line to split successfully");
        // println!("{}, {}", values.0, values.1);
        let first = values.0.trim().parse::<i32>().unwrap();
        let second = values.1.trim().parse::<i32>().unwrap();
        column1.insert(counter, first);
        column2.insert(counter, second);
        counter += 1;
    });
}
pub fn part1(input_path: &str) {
    println!("Part 1, Day 1");
    let input = fs::read_to_string(input_path)
        .expect("Expected to read the file");
    let mut loc1: Vec<i32> = Vec::new();
    let mut loc2: Vec<i32> = Vec::new();
    parse_input(&input, &mut loc1, &mut loc2);
    let mut sum = 0;
    for i in 0..loc1.len() {
        let diff = (loc1[i] - loc2[i]).abs();
        // println!("1: {}, 2: {}, d: {}", loc1[i], loc2[i], diff);
        sum += diff;
    }
    println!("Solution: {}", sum);
}
pub fn part2(input_path: &str) {
    println!("Part 2, Day 1");
    let input = fs::read_to_string(input_path)
        .expect("Expected to read file normally");
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    parse_input(&input, &mut list1, &mut list2);
    let mut similarity_score_sum = 0;
    for elem in &list1 {
        let mut occurences = 0;
        for i in &list2 {
            if elem == i {
                occurences += 1;
            }
        }
        similarity_score_sum += elem * occurences;
    }
    println!("Solution: {}", similarity_score_sum);
}
