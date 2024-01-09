use std::collections::HashSet;

pub fn day3(input: &str) -> (u32, u32) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> u32 {
    let data = input
        .trim()
        .split('\n')
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    data.iter().enumerate().for_each(|(_, row)| {
        row.iter().enumerate().for_each(|(_, col)| print!("{col}"));

        println!()
    });

    let d = data
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(col_index, col)| {
                    if !col.is_ascii_digit() && *col != '.' {
                        println!("{col}");

                        // Find any adjencent number
                        let result = (-1..=1)
                            .flat_map(|drow| {
                                let found = (-1..=1)
                                    .filter_map(|dcol| {
                                        let drow = row_index.checked_add_signed(drow).unwrap_or(0);

                                        let dcol = col_index.checked_add_signed(dcol).unwrap_or(0);

                                        let symbol = data[drow][dcol];
                                        println!(
                                            "Looking at {}, {}, found: {}",
                                            drow,
                                            dcol,
                                            symbol.is_ascii_digit()
                                        );

                                        symbol.is_ascii_digit().then_some((drow, dcol))
                                    })
                                    .collect::<Vec<_>>();

                                println!("{:?}", found);
                                found
                            })
                            .collect::<Vec<_>>();

                        println!(" Has neighbours: {:?}", result);

                        let start_array = result
                            .iter()
                            .map(|p| {
                                let row = p.0;
                                let mut col = p.1;

                                loop {
                                    if let Some(new_col) = col.checked_sub(1) {
                                        println!("Checking:{row}, {new_col}");

                                        if !data[row][new_col].is_ascii_digit() {
                                            break;
                                        }
                                        col = new_col;
                                    } else {
                                        break;
                                    }
                                }
                                (row, col)
                            })
                            .collect::<HashSet<_>>();

                        println!("Start positions: {:?}", start_array);
                        let numbers = start_array
                            .iter()
                            .map(|p| {
                                let row = p.0;
                                let mut col = p.1;
                                let length = data[0].len();
                                let mut number = String::new();
                                println!(
                                    "Looking for {row}, {col} => {}",
                                    data[row][col].is_ascii_digit()
                                );
                                while col < length && data[row][col].is_ascii_digit() {
                                    println!("Checking:{row}, {col}");
                                    number.push(data[row][col]);
                                    println!("String number: {}", number);
                                    col += 1;
                                }
                                number.parse().unwrap_or(0)
                            })
                            .collect::<Vec<_>>();
                        println!("Numbers {:?}", numbers);

                        Some(numbers)
                        //                println!("Has neighbours: {}", result.len() > 0)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashSet<_>>();
    println!("{d:?}");

    //    println!("Sum: {}", d.iter().sum::<u32>());
    d.iter().sum::<u32>()
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

        // #[test]
        // fn test_real_input() {
        //     let input = read_input(3);
        //     let result = part1(&input);
        //     assert_eq!(result, 525119)
        // }
    }
}
