use std::fs::read_to_string;

fn main() {
    let path = "input/1.txt";
    let input = read_to_string(path).unwrap();
    let input: Vec<&str> = input.trim().split("\n").collect();

    let sum = input.iter().map(|i| calc_number(i)).sum::<u32>();
    println!("{sum}");

    let numbers = input
        .iter()
        .map(|l| {
            let numbers = l
                .chars()
                .enumerate()
                .map(|(i, ch)| match ch {
                    ch if ch.is_digit(10) => ch.to_digit(10).unwrap(),
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
        .sum::<u32>();

    println!("{numbers}")
}

fn calc_number(input: &str) -> u32 {
    let numbers = input
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    (numbers[0] * 10) + numbers[numbers.len() - 1]
}

#[cfg(test)]
mod tests {
    use crate::calc_number;

    #[test]
    fn test() {
        let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        let sum = input.iter().map(|i| calc_number(i)).sum::<u32>();
        assert_eq!(sum, 142)
    }

    #[test]
    fn test_parse() {
        let input = "1abc2";
        let sum = calc_number(input);
        assert_eq!(sum, 12);
    }

    #[test]
    fn test_with_numbers() {
        let input = "two1nine";

        let numbers = input
            .chars()
            .enumerate()
            .map(|(i, ch)| match ch {
                ch if ch.is_digit(10) => ch.to_digit(10).unwrap(),
                _ => match &input[i..] {
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
        let sum = (numbers[0] * 10) + numbers[numbers.len() - 1];

        assert_eq!(sum, 29)
    }
}
