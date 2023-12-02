fn main() {
    println!("Hello, AoC!");
}


mod dayX {
    pub fn part1(input: X) -> X) {

    }
    pub fn part2(input: X) -> X) {

    }
}



struct TestCase<T,U> {
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
                input: "",
                expected: "",
            },
        ];

        for test_case in test_cases {
            let result = dayX::part1(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }

    #[test]
    fn test_day2() {
        let test_cases = vec![
            TestCase {
                input: "",
                expected: "",
            },
        ];

        for test_case in test_cases {
            let result = dayX::part2(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }
}
