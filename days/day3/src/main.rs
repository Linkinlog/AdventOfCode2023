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
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x].is_numeric() {
                let mut num = grid[y][x].to_digit(10).unwrap() as u32;
                let mut next_x = x + 1;
                while next_x < grid[y].len() && grid[y][next_x].is_numeric() {
                    num = num * 10 + grid[y][next_x].to_digit(10).unwrap();
                    next_x += 1;
                }

                if next_x != x + 1 {
                    continue;
                }

                if check_left(&grid, x, y)
                    || check_right(&grid, x, y)
                    || check_up(&grid, x, y)
                    || check_down(&grid, x, y)
                    || check_up_left(&grid, x, y)
                    || check_up_right(&grid, x, y)
                    || check_down_left(&grid, x, y)
                    || check_down_right(&grid, x, y)
                {
                    count += num;
                }
            }
        }
    }

    count
}

fn check_left(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x == 0 {
        return false;
    }
    let mut i = x - 1;
    while i > 0 || i == 0 {
        if grid[y][i] == '#' || grid[y][i] == '*' {
            return true;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    false
}

fn check_right(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x == grid[y].len() - 1 {
        return false;
    }
    let mut i = x + 1;
    while i < grid[y].len() {
        if grid[y][i] == '#' || grid[y][i] == '*' {
            return true;
        }
        i += 1;
    }
    false
}

fn check_up(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y == 0 {
        return false;
    }
    let mut i = y - 1;
    while i > 0 {
        if grid[i][x] == '#' || grid[i][x] == '*' {
            return true;
        }
        i -= 1;
    }
    false
}

fn check_down(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y == grid.len() - 1 {
        return false;
    }
    let mut i = y + 1;
    while i < grid.len() {
        if grid[i][x] == '#' || grid[i][x] == '*' {
            return true;
        }
        i += 1;
    }
    false
}

fn check_up_left(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 {
        return false;
    }
    let mut i = x - 1;
    let mut j = y - 1;
    while i > 0 && j > 0 {
        if grid[j][i] == '#' || grid[j][i] == '*' {
            return true;
        }
        i -= 1;
        j -= 1;
    }
    false
}

fn check_up_right(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x == grid[y].len() - 1 || y == 0 {
        return false;
    }
    let mut i = x + 1;
    let mut j = y - 1;
    while i < grid[y].len() && j > 0 {
        if grid[j][i] == '#' || grid[j][i] == '*' {
            return true;
        }
        i += 1;
        j -= 1;
    }
    false
}

fn check_down_left(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x == 0 || y == grid.len() - 1 {
        return false;
    }
    let mut i = x - 1;
    let mut j = y + 1;
    while (i > 0 || i == 0) && j < grid.len() {
        if grid[j][i] == '#' || grid[j][i] == '*' {
            return true;
        }
        if i == 0 {
            break;
        }
        i -= 1;
        j += 1;
    }
    false
}

fn check_down_right(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x == grid[y].len() - 1 || y == grid.len() - 1 {
        return false;
    }
    let mut i = x + 1;
    let mut j = y + 1;
    while i < grid[y].len() && j < grid.len() {
        if grid[j][i] == '#' || grid[j][i] == '*' {
            return true;
        }
        i += 1;
        j += 1;
    }
    false
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
                input: "*111",
                expected: 111,
            },
            TestCase {
                input: "2*",
                expected: 2,
            },
            TestCase {
                input: ".3........
*.........",
                expected: 3,
            },
            TestCase {
                input: ".4........
...*......",
                expected: 0,
            },
            TestCase {
                input: ".5........
..#.......",
                expected: 5,
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
