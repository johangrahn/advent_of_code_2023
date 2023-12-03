use std::collections::HashMap;

pub fn day2(input: &str) -> (u32, u32) {
    (part1(input), 2)
}

fn part1(input: &str) -> u32 {
    let input: Vec<&str> = input.trim().split('\n').collect();

    let sum = input
        .iter()
        .filter_map(|game| get_success(game))
        .sum::<u32>();

    sum
}

fn get_success(input: &str) -> Option<u32> {
    let max = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let game: Vec<&str> = input.split(':').collect();
    let game_number = game[0];
    let game_number = game_number.rsplit_once(' ').unwrap();
    let game_number = game_number.1.parse().unwrap();

    let result = game[1]
        .trim()
        .split(';')
        .map(|s| {
            s.trim()
                .split(',')
                .map(|cubes| {
                    let cubes = cubes.trim();
                    let parts = cubes.split_once(' ').unwrap();
                    let number: u32 = parts.0.parse().unwrap();
                    let t = parts.1;
                    max[t] >= number
                })
                .filter(|f| !(*f))
                .count()
                == 0
        })
        .filter(|f| !(*f))
        .count()
        == 0;

    result.then_some(game_number)
}
