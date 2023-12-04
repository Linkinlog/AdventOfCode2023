use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        println!(
            "{}",
            part2(lines.map(|x| x.unwrap()).collect::<Vec<String>>())
        );
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(input: Vec<String>, critera: Pull) -> u32 {
    let mut sum = 0;
    for line in input {
        let game = parse_game(line);
        if check_criteria(&game.pulls, &critera) {
            sum += game.id;
        }
    }
    sum
}

fn part2(input: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in input {
        let game = parse_game(line);
        let lowest_p = lowest_possible_pull(&game.pulls);
        sum += power(lowest_p)
    }
    sum
}

fn check_criteria(pulls: &Vec<Pull>, critera: &Pull) -> bool {
    pulls.iter().all(|pull| pull.blue <= critera.blue)
        && pulls.iter().all(|pull| pull.red <= critera.red)
        && pulls.iter().all(|pull| pull.green <= critera.green)
}

fn lowest_possible_pull(pulls: &Vec<Pull>) -> Pull {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for pull in pulls {
        if pull.red > red {
            red = pull.red
        };
        if pull.blue > blue {
            blue = pull.blue
        };
        if pull.green > green {
            green = pull.green
        };
    }

    Pull { red, blue, green }
}

fn power(pull: Pull) -> u32 {
    pull.red * pull.green * pull.blue
}

fn parse_game(line: String) -> Game {
    let parts = line.split(": ").collect::<Vec<&str>>();
    let id = parts[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();
    let pulls = parts[1]
        .split("; ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|pull| parse_pull(pull))
        .collect::<Vec<Pull>>();
    Game { id, pulls }
}

fn parse_pull(pull: &str) -> Pull {
    let parts = pull.split(", ").collect::<Vec<&str>>();
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;
    for part in parts {
        let color = part.split(" ").collect::<Vec<&str>>()[1];
        let count = part.split(" ").collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap();
        match color {
            "blue" => blue += count,
            "red" => red += count,
            "green" => green += count,
            _ => panic!("Invalid color"),
        }
    }
    Pull { blue, red, green }
}

struct Pull {
    blue: u32,
    red: u32,
    green: u32,
}

struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

#[test]
fn test_part1() {
    struct TestCase<T, U, J> {
        input: T,
        criteria: U,
        expected: J,
    }

    let test_cases = vec![TestCase {
        input: vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ],
        criteria: Pull {
            blue: 14,
            green: 13,
            red: 12,
        },
        expected: 8,
    }];

    for test_case in test_cases {
        let result = part1(test_case.input, test_case.criteria);
        assert_eq!(result, test_case.expected);
    }
}

#[test]
fn test_part2() {
    struct TestCase<T, J> {
        input: T,
        expected: J,
    }

    let test_cases = vec![
        TestCase {
            input: vec!["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()],
            expected: 48,
        },
        TestCase {
            input: vec![
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            ],
            expected: 12,
        },
        TestCase {
            input: vec![
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                    .to_string(),
            ],
            expected: 1560,
        },
        TestCase {
            input: vec![
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                    .to_string(),
            ],
            expected: 630,
        },
        TestCase {
            input: vec!["Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()],
            expected: 36,
        },
        TestCase {
            input: vec![
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                    .to_string(),
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                    .to_string(),
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
            ],
            expected: 2286,
        },
    ];

    for test_case in test_cases {
        let result = part2(test_case.input);
        assert_eq!(result, test_case.expected);
    }
}
