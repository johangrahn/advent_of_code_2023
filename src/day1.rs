pub fn day1(_input: &str) -> (u32, u32) {
    (1, 1)
}

// // fn main_day1() {
// //     let path = "input/1.txt";
// //     let input = read_to_string(path).unwrap();
// //     let input: Vec<&str> = input.trim().split("\n").collect();

// //     let sum = input.iter().map(|i| calc_number(i)).sum::<u32>();
// //     println!("{sum}");

// //     let numbers = input
// //         .iter()
// //         .map(|l| {
// //             let numbers = l
// //                 .chars()
// //                 .enumerate()
// //                 .map(|(i, ch)| match ch {
// //                     ch if ch.is_digit(10) => ch.to_digit(10).unwrap(),
// //                     _ => match &l[i..] {
// //                         s if s.starts_with("one") => 1,
// //                         s if s.starts_with("two") => 2,
// //                         s if s.starts_with("three") => 3,
// //                         s if s.starts_with("four") => 4,
// //                         s if s.starts_with("five") => 5,
// //                         s if s.starts_with("six") => 6,
// //                         s if s.starts_with("seven") => 7,
// //                         s if s.starts_with("eight") => 8,
// //                         s if s.starts_with("nine") => 9,
// //                         _ => 0,
// //                     },
// //                 })
// //                 .filter(|num| *num != 0)
// //                 .collect::<Vec<u32>>();
// //             (numbers[0] * 10) + numbers[numbers.len() - 1]
// //         })
// //         .sum::<u32>();

// //     println!("{numbers}")
// // }

// fn calc_number(input: &str) -> u32 {
//     let numbers = input
//         .chars()
//         .filter(|c| c.is_digit(10))
//         .map(|c| c.to_digit(10).unwrap())
//         .collect::<Vec<_>>();
//     (numbers[0] * 10) + numbers[numbers.len() - 1]
// }

// #[cfg(test)]
// mod tests_day1 {
//     use crate::calc_number;

//     #[test]
//     fn test() {
//         let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

//         let sum = input.iter().map(|i| calc_number(i)).sum::<u32>();
//         assert_eq!(sum, 142)
//     }

//     #[test]
//     fn test_parse() {
//         let input = "1abc2";
//         let sum = calc_number(input);
//         assert_eq!(sum, 12);
//     }

//     #[test]
//     fn test_with_numbers() {
//         let input = "two1nine";

//         let numbers = input
//             .chars()
//             .enumerate()
//             .map(|(i, ch)| match ch {
//                 ch if ch.is_digit(10) => ch.to_digit(10).unwrap(),
//                 _ => match &input[i..] {
//                     s if s.starts_with("one") => 1,
//                     s if s.starts_with("two") => 2,
//                     s if s.starts_with("three") => 3,
//                     s if s.starts_with("four") => 4,
//                     s if s.starts_with("five") => 5,
//                     s if s.starts_with("six") => 6,
//                     s if s.starts_with("seven") => 7,
//                     s if s.starts_with("eight") => 8,
//                     s if s.starts_with("nine") => 9,
//                     _ => 0,
//                 },
//             })
//             .filter(|num| *num != 0)
//             .collect::<Vec<u32>>();
//         let sum = (numbers[0] * 10) + numbers[numbers.len() - 1];

//         assert_eq!(sum, 29)
//     }
// }

// mod tests_day2 {
//     use std::collections::HashMap;

//     #[test]
//     fn test_example() {
//         let input = [
//             "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
//             "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
//             "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
//             "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
//             "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
//         ];

//         let sum = input
//             .iter()
//             .filter_map(|game| get_success(game))
//             .sum::<u32>();
//         assert_eq!(sum, 8)
//     }
// }
