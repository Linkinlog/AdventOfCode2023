fn main() {
    let input = include_str!("input");
    let cards: Vec<String> = input
        .split("\n")
        .map(|x| x.to_owned())
        .filter(|x| !x.is_empty())
        .collect();

    let result = part1(cards.clone());
    let result2 = part2(cards.clone());
    println!("Part 1: {}", result);
    println!("Part 2: {}", result2);
}

fn part1(cards: Vec<String>) -> usize {
    cards
        .iter()
        .flat_map(|card| card.lines())
        .map(parse_game)
        .fold(0, |acc, game| {
            acc + compare_sets_and_add_points((game.winning_numbers, game.our_numbers))
        })
}

fn part2(cards: Vec<String>) -> usize {
    let original_games: Vec<Game> = cards
        .iter()
        .flat_map(|card| card.lines().map(parse_game))
        .collect();

    let mut all_games: Vec<Game> = Vec::new();
    let mut to_process: Vec<Game> = original_games.clone();

    while let Some(game) = to_process.pop() {
        all_games.push(game.clone());

        let matches = compare_sets((game.winning_numbers.clone(), game.our_numbers.clone()));
        let start_index = original_games.iter().position(|g| *g == game).unwrap_or(0);

        for next_game_index in start_index + 1..start_index + 1 + matches {
            if next_game_index < original_games.len() {
                to_process.push(original_games[next_game_index].clone());
            }
        }
    }

    all_games.len()
}

fn compare_sets_and_add_points(input: (Vec<u32>, Vec<u32>)) -> usize {
    let (set_a, set_b) = input;
    set_b
        .iter()
        .filter(|x| set_a.contains(x))
        .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })

}

fn compare_sets(input: (Vec<u32>, Vec<u32>)) -> usize {
    let (set_a, set_b) = input;

    set_b.iter()
        .filter(|x| set_a.contains(x))
        .fold(0, |acc, _| acc + 1)

}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    id: u32,
    winning_numbers: Vec<u32>,
    our_numbers: Vec<u32>,
}

fn parse_game(input: &str) -> Game {
    let mut winning_numbers: Vec<u32> = Vec::new();
    let mut our_numbers: Vec<u32> = Vec::new();

    let mut input = input.split(":");
    let id = input
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    input = input.next().unwrap().split("|");
    let mut winning_numbers_iter = input.next().unwrap().split_whitespace();
    let mut our_numbers_iter = input.next().unwrap().split_whitespace();

    for _ in 0..winning_numbers_iter.clone().count() {
        winning_numbers.push(
            winning_numbers_iter
                .next()
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap(),
        );
    }

    for _ in 0..our_numbers_iter.clone().count() {
        our_numbers.push(
            our_numbers_iter
                .next()
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap(),
        );
    }

    Game {
        id,
        winning_numbers,
        our_numbers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase<T, U> {
        input: T,
        expected: U,
    }

    #[test]
    fn test_compare_sets() {
        let test_cases = vec![
            TestCase {
                input: (vec![1, 2, 3], vec![3, 4, 5]),
                expected: 1,
            },
            TestCase {
                input: (vec![1, 2, 3], vec![3, 2, 5]),
                expected: 2,
            },
            TestCase {
                input: (vec![1, 2, 3], vec![3, 2, 1]),
                expected: 4,
            },
            TestCase {
                input: (vec![1, 2, 3, 4], vec![3, 2, 4, 1]),
                expected: 8,
            },
        ];

        for test_case in test_cases {
            let result = compare_sets_and_add_points(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }

    #[test]
    fn test_parse_game() {
        let test_cases = vec![
            TestCase {
                input: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
                expected: Game {
                    id: 1,
                    winning_numbers: vec![41, 48, 83, 86, 17],
                    our_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                },
            },
            TestCase {
                input: "Card 2:  9  7 37  2  5 |  7  2  5  9 37  8  6  1",
                expected: Game {
                    id: 2,
                    winning_numbers: vec![9, 7, 37, 2, 5],
                    our_numbers: vec![7, 2, 5, 9, 37, 8, 6, 1],
                },
            },
        ];
        for test_case in test_cases {
            let result = parse_game(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }

    #[test]
    fn test_part1() {
        let test_cases = vec![TestCase {
            input: vec![
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_owned(),
                "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_owned(),
                "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_owned(),
                "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_owned(),
                "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_owned(),
                "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_owned(),
            ],
            expected: 13,
        }];

        for test_case in test_cases {
            let result = part1(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = vec![TestCase {
            input: vec![
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_owned(),
                "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_owned(),
                "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_owned(),
                "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_owned(),
                "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_owned(),
                "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_owned(),
            ],
            expected: 30,
        }];

        for test_case in test_cases {
            let result = part2(test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }
}
