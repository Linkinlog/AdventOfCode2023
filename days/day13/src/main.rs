use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    if let Ok(lines) = read_lines("./input") {}
}

fn read_lines(filename: &str) -> io::Result<io::Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(input: Vec<&str>) -> usize {
    // iterate over each line verically and horizontally,
    // if the surrouncing lines are identical, check left/right for smaller number as x
    // lop off lines so that 
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
    fn test_vertical_sums() {
        let test_case = TestCase {
            input: vec![
                "#.##..##.",
                "..#.##.#.",
                "##......#",
                "##......#",
                "..#.##.#.",
                "..##..##.",
                "#.#.##.#.",
            ],
            expected: 5,
        };

        let result = part1(test_case.input);
        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn test_horizontal_sums() {
        let test_case = TestCase {
            input: vec![
                "#...##..#",
                "#....#..#",
                "..##..###",
                "#####.##.",
                "#####.##.",
                "..##..###",
                "#....#..#",
            ],
            expected: 400,
        };

        let result = part1(test_case.input);
        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn test_part2() {
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