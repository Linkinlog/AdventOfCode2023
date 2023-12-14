use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    if let Ok(lines) = read_lines("./input") {
        //
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn part1(input: &str) -> &str {
    input
}

fn part2(input: &str) -> &str {
    input
}



struct TestCase<T,U> {
    input: T,
    expected: U,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_cases = vec![
            TestCase {
                input: "",
                expected: "",
            },
        ];

        for test_case in test_cases {
            let result = part1(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = vec![
            TestCase {
                input: "",
                expected: "",
            },
        ];

        for test_case in test_cases {
            let result = part2(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }
}
