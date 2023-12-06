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
    // turn string into 2d array
    // iterate over each line, find digits
    // check left, right, up, down, and diagonals for symbols
    // if symbol is found, increment count
    // return count
    let mut count = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        println!("{}", line);
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if ['*', '#', '$'].contains(c) {
                count += check_up_for_numbers(&grid, x, y).unwrap_or(0);
                count += check_down_for_numbers(&grid, x, y).unwrap_or(0);
                count += check_left_for_numbers(&grid, x, y).unwrap_or(0);
                count += check_right_for_numbers(&grid, x, y).unwrap_or(0);
            }
        }
    }

    count
}

fn check_left_for_numbers(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Result<u32, u32> {
    if x == 0 {
        return Err(0);
    }
    let mut sum = 0;
    let mut i = x;
    loop {
        i -= 1;
        if grid[y][i].is_digit(10) {
            sum = format!("{}{}", sum, grid[y][i]).parse::<u32>().unwrap();
        } else {
            break;
        }
        if i == 0 {
            break;
        }
    }
    if sum == 0 {
        return Err(0);
    }
    Ok(sum)
}

fn check_right_for_numbers(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Result<u32, u32> {
    if x == grid[y].len() - 1 {
        return Err(0);
    }
    let mut sum = 0;
    let mut i = x;
    loop {
        i += 1;
        if grid[y][i].is_digit(10) {
            sum = format!("{}{}", sum, grid[y][i]).parse::<u32>().unwrap();
        } else {
            break;
        }
        if i == grid[y].len() - 1 {
            break;
        }
    }
    if sum == 0 {
        return Err(0);
    }
    Ok(sum)
}

fn check_up_for_numbers(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Result<u32, u32> {
    if y == 0 {
        return Err(0);
    }

    let mut total_sum = 0;

    if x > 0 && grid[y - 1][x - 1].is_digit(10) {
        total_sum += check_left_for_numbers(grid, x - 1, y - 1).unwrap_or(0);
    }

    if grid[y - 1][x].is_digit(10) {
        let left_sum = check_left_for_numbers(grid, x, y - 1).unwrap_or(0);
        let right_sum = check_right_for_numbers(grid, x, y - 1).unwrap_or(0);
        if right_sum > 0 && left_sum > 0 {
        total_sum = (grid[y - 1][x].to_digit(10).unwrap() * 10 + left_sum) * 10 + right_sum
        } else if right_sum > 0 {
            total_sum = (grid[y - 1][x].to_digit(10).unwrap() * 10) + right_sum
        } else if left_sum > 0 {
            total_sum = (grid[y - 1][x].to_digit(10).unwrap() * 10) + left_sum
        } else {
            total_sum = grid[y - 1][x].to_digit(10).unwrap()
        }
    }

    if x < grid[y].len() - 1 && grid[y - 1][x + 1].is_digit(10) {
        total_sum += check_right_for_numbers(grid, x, y - 1).unwrap_or(0);
    }

    if total_sum == 0 {
        return Err(0);
    }

    Ok(total_sum)
}

fn check_down_for_numbers(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Result<u32, u32> {
    if y == grid.len() - 1 {
        return Err(0);
    }

    let mut total_sum = 0;

    if x > 0 && grid[y + 1][x - 1].is_digit(10) {
        total_sum += check_left_for_numbers(grid, x - 1, y + 1).unwrap_or(0);
    }

    if grid[y + 1][x].is_digit(10) {
        let left_sum = check_left_for_numbers(grid, x, y + 1).unwrap_or(0);
        let right_sum = check_right_for_numbers(grid, x, y + 1).unwrap_or(0);
        if right_sum > 0 && left_sum > 0 {
        total_sum = (grid[y + 1][x].to_digit(10).unwrap() * 10 + left_sum) * 10 + right_sum
        } else if right_sum > 0 {
            total_sum = (grid[y + 1][x].to_digit(10).unwrap() * 10) + right_sum
        } else if left_sum > 0 {
            total_sum = (grid[y + 1][x].to_digit(10).unwrap() * 10) + left_sum
        } else {
            total_sum = grid[y + 1][x].to_digit(10).unwrap()
        }
    }

    if x < grid[y].len() - 1 && grid[y + 1][x + 1].is_digit(10) {
        total_sum += check_right_for_numbers(grid, x, y + 1).unwrap_or(0);
    }

    if total_sum == 0 {
        return Err(0);
    }

    Ok(total_sum)
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
