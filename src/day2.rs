use std::collections::HashMap;

pub fn day2(input: &str) -> (u32, u32) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> u32 {
    let input: Vec<&str> = input.trim().split('\n').collect();

    let sum = input
        .iter()
        .filter_map(|game| get_success(game))
        .sum::<u32>();

    sum
}

fn part2(input: &str) -> u32 {
    let input: Vec<&str> = input.trim().split('\n').collect();

    let sum = input.iter().map(|f| get_success2(f)).sum::<u32>();

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

fn get_success2(input: &str) -> u32 {
    let parts = input.split(':').collect::<Vec<_>>();

    let mut counter = HashMap::new();

    parts[1]
        .trim()
        .split([';', ','])
        .map(|cube| {
            let cube = cube.trim();
            println!("{:?}", cube);
            let (num, t) = cube.split_once(' ').unwrap();
            let mut num = num.parse::<u32>().unwrap();
            //println!("{:?} ({num} -> {t})", cube);

            counter
                .entry(t)
                .and_modify(|e| *e = if e < &mut num { num } else { *e })
                .or_insert(num);

            0
        })
        .sum::<u32>();
    let product = counter.values().product::<u32>();
    println!("{:?} -> {product}", counter);
    product
}

#[cfg(test)]
mod day2_tests {
    mod part1 {
        use advent_of_code_2023::read_input;

        use crate::day2::part1;

        #[test]
        fn test_example() {
            let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

            let result = part1(input);
            assert_eq!(result, 8)
        }

        #[test]
        fn test_real_input() {
            let input = read_input(2);
            let result = part1(&input);
            assert_eq!(result, 2207)
        }
    }

    mod part2 {
        use advent_of_code_2023::read_input;

        use crate::day2::part2;

        #[test]
        fn test_example() {
            let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

            let result = part2(input);
            assert_eq!(result, 2286)
        }

        #[test]
        fn test_real_input() {
            let input = read_input(2);
            let result = part2(&input);
            assert_eq!(result, 62241)
        }
    }
}
