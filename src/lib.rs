use std::fs::read_to_string;

pub fn read_input(day: usize) -> String {
    let path = format!("input/{}.txt", day);
    read_to_string(path).unwrap()
}
