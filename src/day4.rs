use std::fs;

pub fn call_day4() -> () {
    let text = fs::read_to_string("input_day4.txt").unwrap();
    let grid = text_to_matrix(&text);

    let number_of_xmas = number_of_mas_x_shape(&grid);

    println!("Number of XMAS : {number_of_xmas}");
}

fn text_to_matrix(text: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for line in text.lines() {
        let grid_line = line.chars().collect::<Vec<char>>();
        grid.push(grid_line);
    }

    grid
}

fn number_of_mas_x_shape(grid: &Vec<Vec<char>>) -> i32 {
    let mut number_of_xmas = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let letter = grid[i][j];
            if letter == 'A' && is_mas_x_shape(&grid, i, j) {
                number_of_xmas += 1;
            }
        }
    }

    number_of_xmas
}

fn number_of_xmas_around(grid: &Vec<Vec<char>>, pos_x: usize, pos_y: usize) -> i32 {
    let mut result = 0;
    let length = grid.len();

    // TOP
    if pos_x >= 3 {
        if grid[pos_x - 1][pos_y] == 'M'
            && grid[pos_x - 2][pos_y] == 'A'
            && grid[pos_x - 3][pos_y] == 'S'
        {
            result += 1;
        }
    }

    // BOTTOM
    if pos_x <= length - 4 {
        if grid[pos_x + 1][pos_y] == 'M'
            && grid[pos_x + 2][pos_y] == 'A'
            && grid[pos_x + 3][pos_y] == 'S'
        {
            result += 1;
        }
    }

    // LEFT
    if pos_y >= 3 {
        if grid[pos_x][pos_y - 1] == 'M'
            && grid[pos_x][pos_y - 2] == 'A'
            && grid[pos_x][pos_y - 3] == 'S'
        {
            result += 1;
        }
    }

    // RIGHT
    if pos_y <= length - 4 {
        if grid[pos_x][pos_y + 1] == 'M'
            && grid[pos_x][pos_y + 2] == 'A'
            && grid[pos_x][pos_y + 3] == 'S'
        {
            result += 1;
        }
    }

    // TOP LEFT
    if pos_x >= 3 && pos_y >= 3 {
        if grid[pos_x - 1][pos_y - 1] == 'M'
            && grid[pos_x - 2][pos_y - 2] == 'A'
            && grid[pos_x - 3][pos_y - 3] == 'S'
        {
            result += 1;
        }
    }

    // TOP RIGHT
    if pos_x >= 3 && pos_y <= length - 4 {
        if grid[pos_x - 1][pos_y + 1] == 'M'
            && grid[pos_x - 2][pos_y + 2] == 'A'
            && grid[pos_x - 3][pos_y + 3] == 'S'
        {
            result += 1;
        }
    }

    // BOTTOM LEFT
    if pos_x <= length - 4 && pos_y >= 3 {
        if grid[pos_x + 1][pos_y - 1] == 'M'
            && grid[pos_x + 2][pos_y - 2] == 'A'
            && grid[pos_x + 3][pos_y - 3] == 'S'
        {
            result += 1;
        }
    }

    // BOTTOM RIGHT
    if pos_x <= length - 4 && pos_y <= length - 4 {
        if grid[pos_x + 1][pos_y + 1] == 'M'
            && grid[pos_x + 2][pos_y + 2] == 'A'
            && grid[pos_x + 3][pos_y + 3] == 'S'
        {
            result += 1;
        }
    }

    result
}

fn is_mas_x_shape(grid: &Vec<Vec<char>>, pos_x: usize, pos_y: usize) -> bool {
    let length = grid.len();
    let accepted_range = 1..(length - 1);

    if accepted_range.contains(&pos_x) && accepted_range.contains(&pos_y) {
        if grid[pos_x - 1][pos_y - 1] == 'M' && grid[pos_x + 1][pos_y + 1] == 'S' {
            if grid[pos_x + 1][pos_y - 1] == 'M' && grid[pos_x - 1][pos_y + 1] == 'S' {
                return true;
            } else if grid[pos_x - 1][pos_y + 1] == 'M' && grid[pos_x + 1][pos_y - 1] == 'S' {
                return true;
            }
        } else if grid[pos_x + 1][pos_y + 1] == 'M' && grid[pos_x - 1][pos_y - 1] == 'S' {
            if grid[pos_x + 1][pos_y - 1] == 'M' && grid[pos_x - 1][pos_y + 1] == 'S' {
                return true;
            } else if grid[pos_x - 1][pos_y + 1] == 'M' && grid[pos_x + 1][pos_y - 1] == 'S' {
                return true;
            }
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_xmas_around() {
        let input = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S'],
            vec!['A', 'M', 'X', 'M', 'A', 'S', 'A'],
            vec!['M', 'S', 'M', 'M', 'A', 'S', 'M'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X'],
            vec!['X', 'X', 'S', 'M', 'M', 'S', 'X'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S'],
        ];
        assert_eq!(number_of_xmas_around(&input, 4, 0), 1);
        assert_eq!(number_of_xmas_around(&input, 0, 4), 0);
        assert_eq!(number_of_xmas_around(&input, 2, 2), 3);
    }

    #[test]
    fn test_is_mas_x_shape() {
        let input = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S'],
            vec!['A', 'M', 'X', 'S', 'A', 'S', 'A'],
            vec!['M', 'S', 'M', 'M', 'A', 'S', 'M'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X'],
            vec!['X', 'X', 'S', 'M', 'M', 'S', 'X'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S'],
        ];
        assert_eq!(is_mas_x_shape(&input, 1, 2), true);
        assert_eq!(is_mas_x_shape(&input, 2, 4), false);
    }

    #[test]
    fn test_number_of_mas_x_shape() {
        let test_string = "MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX";

        let grid = text_to_matrix(test_string);
        assert_eq!(number_of_mas_x_shape(&grid), 9);
    }
}
