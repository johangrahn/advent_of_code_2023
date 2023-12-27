use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn day3(input: &str) -> (u32, u32) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> u32 {
    let data = input
        .trim()
        .split('\n')
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut mapper: HashMap<(usize, usize), char> = HashMap::new();

    for (r, row) in data.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if col.is_ascii_digit() {
                mapper.insert((r, c), *col);
            }

            if !col.is_ascii_digit() && *col != '.' {
                for dy in [-1, 0, 1] {
                    for dx in [-1, 0, 1] {
                        let y = r as i32 + dy;
                        let mut x = c as i32 + dx;

                        if dy == 0 && dx == 0 {
                            continue;
                        }

                        let curr_char =
                            data[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()];

                        if y < 0
                            || y >= (data.len() as i32)
                            || x < 0
                            || x >= (row.len() as i32)
                            || !curr_char.is_ascii_digit()
                        {
                            continue;
                        }

                        while x > 0
                            && data[usize::try_from(y).unwrap()][usize::try_from(x - 1).unwrap()]
                                .is_ascii_digit()
                        {
                            x -= 1;
                        }

                        set.insert((y as usize, x as usize));
                    }
                }
            }
        }
    }
    // println!("{set:?}");
    // println!("{mapper:?}");

    let length = data[0].len();

    set.iter()
        .map(|(y, x)| {
            let mut stop = *x;

            while stop < length
                && mapper.get(&(*y, stop)).is_some()
                && mapper.get(&(*y, stop)).unwrap().is_ascii_digit()
            {
                stop += 1;
            }

            String::from_iter(data[*y][*x..stop].to_vec())
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
}
fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod day3_tests {
    const TEST_INPUT: &str = "
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755. 
    ...$.*....
    .664.598..";

    mod part1 {
        use crate::day3::{day3_tests::TEST_INPUT, part1};
        use advent_of_code_2023::read_input;

        #[test]
        fn test_example() {
            let result = part1(TEST_INPUT);
            assert_eq!(result, 4361)
        }

        #[test]
        fn test_real_input() {
            let input = read_input(3);
            let result = part1(&input);
            assert_eq!(result, 525119)
        }
    }
}
