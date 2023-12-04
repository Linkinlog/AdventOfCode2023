use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        println!(
            "{}",
            day1::part2(lines.map(|x| x.unwrap()).collect::<Vec<String>>())
        );
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

mod day1 {
    pub fn part1(inputs: Vec<String>) -> i32 {
        let mut total = 0;
        for input in inputs {
            total += get_digits(input);
        }
        return total;
    }

    pub fn part2(inputs: Vec<String>) -> i32 {
        let mut total = 0;
        for input in inputs {
            total += find_first_and_last_number(&input).parse::<i32>().unwrap();
        }
        return total;
    }

    fn get_digits(input: String) -> i32 {
        let sum = input
            .chars()
            .filter(|x| x.is_digit(10))
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        let sumstr = sum.to_string();
        let first_digit = sumstr.chars().nth(0).unwrap();
        let last_digit = sumstr.chars().nth(sumstr.len() - 1).unwrap();
        let concatenated = format!("{}{}", first_digit, last_digit)
            .parse::<i32>()
            .unwrap();
        return concatenated;
    }
    use regex::Regex;
    use std::collections::HashMap;

    fn find_first_and_last_number(text: &str) -> String {
        let words: HashMap<&str, &str> = [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
            ("eno", "1"),
            ("owt", "2"),
            ("eerht", "3"),
            ("ruof", "4"),
            ("evif", "5"),
            ("xis", "6"),
            ("neves", "7"),
            ("thgie", "8"),
            ("enin", "9"),
        ]
        .iter()
        .cloned()
        .collect();

        let re = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

        let mut first_number = "0";
        let mut last_number = "0";
        let mut first_found = false;

        for cap in re.captures_iter(text) {
            let num_str = cap.get(0).unwrap().as_str();
            let num = if let Some(&n) = words.get(num_str) {
                n
            } else {
                num_str
            };

            if !first_found {
                first_number = num;
                first_found = true;
            }

            last_number = num; // This will keep updating until the last match
        }

        return format!("{}{}", first_number, last_number);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        struct TestCase<T, U> {
            input: T,
            expected: U,
        }
        let test_cases = vec![
            TestCase {
                input: vec!["1abc2".to_string()],
                expected: 12,
            },
            TestCase {
                input: vec!["pqr3stu8vwx".to_string()],
                expected: 38,
            },
            TestCase {
                input: vec!["a1b2c3d4e5f".to_string()],
                expected: 15,
            },
            TestCase {
                input: vec!["treb7uchet".to_string()],
                expected: 77,
            },
            TestCase {
                input: vec![
                    "1abc2".to_string(),
                    "pqr3stu8vwx".to_string(),
                    "a1b2c3d4e5f".to_string(),
                    "treb7uchet".to_string(),
                ],
                expected: 142,
            },
        ];

        for test_case in test_cases {
            let result = day1::part1(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }
    #[test]
    fn test_part2() {
        struct TestCase<T, U> {
            input: T,
            expected: U,
        }
        let test_cases = vec![
            TestCase {
                input: vec!["two1nine".to_string()],
                expected: 29,
            },
            TestCase {
                input: vec!["eightwothree".to_string()],
                expected: 83,
            },
            TestCase {
                input: vec!["abcone2threexyz".to_string()],
                expected: 13,
            },
            TestCase {
                input: vec!["xtwone3four".to_string()],
                expected: 24,
            },
            TestCase {
                input: vec!["4nineeightseven2".to_string()],
                expected: 42,
            },
            TestCase {
                input: vec!["zoneight234".to_string()],
                expected: 14,
            },
            TestCase {
                input: vec!["7pqrstsixteen".to_string()],
                expected: 76,
            },
            TestCase {
                input: vec![
                    "two1nine".to_string(),
                    "eightwothree".to_string(),
                    "abcone2threexyz".to_string(),
                    "xtwone3four".to_string(),
                    "4nineeightseven2".to_string(),
                    "zoneight234".to_string(),
                    "7pqrstsixteen".to_string(),
                ],
                expected: 281,
            },
        ];

        for test_case in test_cases {
            let result = day1::part2(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }
}
