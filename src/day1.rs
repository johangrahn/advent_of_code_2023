pub fn day1(input: &str) -> (u32, u32) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> u32 {
    let input: Vec<&str> = input.trim().split('\n').collect();

    input.iter().map(|i| calc_number(i)).sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let input: Vec<&str> = input.trim().split('\n').collect();

    input
        .iter()
        .map(|l| {
            let numbers = l
                .chars()
                .enumerate()
                .map(|(i, ch)| match ch {
                    ch if ch.is_ascii_digit() => ch.to_digit(10).unwrap(),
                    _ => match &l[i..] {
                        s if s.starts_with("one") => 1,
                        s if s.starts_with("two") => 2,
                        s if s.starts_with("three") => 3,
                        s if s.starts_with("four") => 4,
                        s if s.starts_with("five") => 5,
                        s if s.starts_with("six") => 6,
                        s if s.starts_with("seven") => 7,
                        s if s.starts_with("eight") => 8,
                        s if s.starts_with("nine") => 9,
                        _ => 0,
                    },
                })
                .filter(|num| *num != 0)
                .collect::<Vec<u32>>();
            (numbers[0] * 10) + numbers[numbers.len() - 1]
        })
        .sum::<u32>()
}

fn calc_number(input: &str) -> u32 {
    let numbers = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    (numbers[0] * 10) + numbers[numbers.len() - 1]
}

#[cfg(test)]
mod day1_tests {

    mod part1 {
        use advent_of_code_2023::read_input;

        use crate::day1::part1;

        #[test]
        fn test_example() {
            let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f 
            treb7uchet
            ";

            let result = part1(input);
            assert_eq!(result, 142)
        }

        #[test]
        fn test_real_input() {
            let input = read_input(1);
            let result = part1(&input);
            assert_eq!(result, 55816)
        }
    }

    mod part2 {
        use advent_of_code_2023::read_input;

        use crate::day1::part2;

        #[test]
        fn test_example() {
            let input = "two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen";
            let result = part2(input);
            assert_eq!(result, 281)
        }

        #[test]
        fn test_real_input() {
            let input = read_input(1);
            let result = part2(&input);
            assert_eq!(result, 54980)
        }
    }
}
