use std::{collections::HashMap, fs::read_to_string};

mod day1;
use day1::day1;

mod day2;
use day2::day2;

mod day3;
use day3::day3;

type DayFunction = fn(input: &str) -> (u32, u32);

fn main() {
    let day = 1;
    let input = format!("input/{day}.txt");
    let input = read_to_string(input).unwrap();
    let mut day_functions: HashMap<i32, DayFunction> = HashMap::default();
    day_functions.insert(1, day1);
    day_functions.insert(2, day2);
    day_functions.insert(3, day3);

    let result = day_functions.get(&day).unwrap()(&input);

    println!("{:?}", result);
}
