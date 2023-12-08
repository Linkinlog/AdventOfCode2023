use std::collections::HashSet;
use std::fs::File;
use std::io;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        //
    }
}

fn read_lines(filename: &str) -> io::Result<io::BufReader<File>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn part1(input: String) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    let mut symbols = HashSet::new();

    // Identify symbols
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if is_symbol(c) {
                symbols.insert((x, y));
            }
        }
    }

    // Check adjacent cells of symbols for numbers
    for &(x, y) in &symbols {
        sum += check_adjacent_cells_for_numbers(&grid, x, y);
    }

    sum
}

fn is_symbol(c: char) -> bool {
    ['*', '#', '$', '+'].contains(&c)
}

fn check_adjacent_cells_for_numbers(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut adjacent_sum = 0;
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for (dx, dy) in directions {
        let adj_x = x as isize + dx;
        let adj_y = y as isize + dy;

        if adj_x >= 0
            && adj_y >= 0
            && adj_y < grid.len() as isize
            && adj_x < grid[adj_y as usize].len() as isize
        {
            let adj_cell = grid[adj_y as usize][adj_x as usize];
            if adj_cell.is_digit(10) {
                adjacent_sum += extract_number_from_start(grid, adj_x as usize, adj_y as usize);
            }
        }
    }

    adjacent_sum
}

fn extract_number_from_start(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut num = 0;
    let mut x_pos = x;

    while x_pos < grid[y].len() && grid[y][x_pos].is_digit(10) {
        num = num * 10 + grid[y][x_pos].to_digit(10).unwrap();
        x_pos += 1;
    }

    num
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
                input: "111*",
                expected: 111,
            },
            TestCase {
                input: "*222",
                expected: 222,
            },
            TestCase {
                input: ".33.......
*.........",
                expected: 33,
            },
            TestCase {
                input: "44........
...*......",
                expected: 0,
            },
            TestCase {
                input: ".55.......
..#.......",
                expected: 55,
            },
            TestCase {
                input: ".*........
.66.......",
                expected: 66,
            },
            TestCase {
                input: ".*........
...7......",
                expected: 0,
            },
            TestCase {
                input: ".*........
..88......",
                expected: 8,
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
            let result = part1(test_case.input.to_owned());
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
