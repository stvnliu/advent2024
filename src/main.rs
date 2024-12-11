use std::{
    env,
    fs,
};
mod day1;
mod day2;
fn main() {
    day1::part1("./day1.input");
    day1::part2("./day1.input");
    day2::part1("./day2.input");
    day2::part2("./day2.input");
}
