use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    if let Ok(lines) = read_lines("./input") {
        //
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(input: Vec<String>) -> u32 {
    // iterate over the strings 2 at a time
    // find numbers
    // find start/end index of numbers
    // find symbols
    // find index of symbols
    // see if they intersect
    0
}


fn part2(input: &str) -> &str {
    input
}

struct TestCase<T, U> {
    input: T,
    expected: U,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let test_cases = vec![
            TestCase {
                input: "1*",
                expected: 1,
            },
            TestCase {
                input: "*1",
                expected: 1,
            },
            TestCase {
                input: ".2........
*.........",
                expected: 2,
            },
            TestCase {
                input: ".2........
...*......",
                expected: 0,
            },
            TestCase {
                input: ".1........
..#.......",
                expected: 1,
            },
            TestCase {
                input: "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
                expected: 4361,
            },
        ];

        for test_case in test_cases {
            let result = part1(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }

    #[test]
    fn test_day2() {
        let test_cases = vec![TestCase {
            input: "",
            expected: "",
        }];

        for test_case in test_cases {
            let result = part2(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }
}
