pub fn call_day6() -> () {
    let mut grid = read_input();
    let mut grid_part2 = grid.clone();

    let visited_positions = predict_guard_route(&mut grid);
    println!("Number of distincts visited positions : {visited_positions}");

    let obstruction_positions = brute_force_possibilites_for_loop(&mut grid_part2);
    println!("Number of possibilities for obstruction position : {obstruction_positions}");
}

fn read_input() -> Vec<Vec<GridPositionState>> {
    let mut grid = vec![];
    if let Ok(lines) = super::read_lines("input_day6.txt") {
        for line in lines.flatten() {
            let mut row = vec![];
            for c in line.chars() {
                match c {
                    '#' => row.push(GridPositionState::Obstacle),
                    '.' => row.push(GridPositionState::None),
                    '^' => row.push(GridPositionState::Guard(Orientation::Up)),
                    _ => continue,
                }
            }

            grid.push(row);
        }
    }

    grid
}

fn predict_guard_route(grid: &mut Vec<Vec<GridPositionState>>) -> i32 {
    let mut guard_outside_grid = false;
    let mut visited_positions = 0;
    let (mut guard_i, mut guard_j, mut orientation) = find_guard(&grid);
    while !guard_outside_grid {
        if let Some((next_i, next_j)) = next_position(guard_i, guard_j, &orientation, grid.len()) {
            match grid[next_i][next_j] {
                GridPositionState::None => {
                    grid[guard_i][guard_j] = GridPositionState::Visited;
                    grid[next_i][next_j] = GridPositionState::Guard(orientation.clone());
                    visited_positions += 1;
                    guard_i = next_i;
                    guard_j = next_j;
                }
                GridPositionState::Visited => {
                    grid[guard_i][guard_j] = GridPositionState::Visited;
                    grid[next_i][next_j] = GridPositionState::Guard(orientation.clone());
                    guard_i = next_i;
                    guard_j = next_j;
                }
                GridPositionState::Obstacle => {
                    let next_orientation = orientation.next_orientation();
                    grid[guard_i][guard_j] = GridPositionState::Guard(next_orientation);
                    orientation = next_orientation;
                }
                _ => panic!(), // Shouldn't happen
            }
        } else {
            guard_outside_grid = true;
            grid[guard_i][guard_j] = GridPositionState::Visited;
            visited_positions += 1;
        }
    }

    visited_positions
}

fn find_guard(grid: &Vec<Vec<GridPositionState>>) -> (usize, usize, Orientation) {
    for (i, row) in grid.iter().enumerate() {
        for (j, state) in row.iter().enumerate() {
            if let GridPositionState::Guard(orientation) = state {
                return (i, j, orientation.clone());
            }
        }
    }

    panic!()
}

fn next_position(
    current_i: usize,
    current_j: usize,
    orientation: &Orientation,
    grid_size: usize,
) -> Option<(usize, usize)> {
    let mut next_i = current_i;
    let mut next_j = current_j;
    match orientation {
        Orientation::Up => {
            if current_i == 0 {
                return None;
            } else {
                next_i = current_i - 1;
            }
        }
        Orientation::Down => {
            if current_i == (grid_size - 1) {
                return None;
            } else {
                next_i = current_i + 1;
            }
        }
        Orientation::Left => {
            if current_j == 0 {
                return None;
            } else {
                next_j = current_j - 1;
            }
        }
        Orientation::Right => {
            if current_j == (grid_size - 1) {
                return None;
            } else {
                next_j = current_j + 1;
            }
        }
    }

    return Some((next_i, next_j));
}

fn brute_force_possibilites_for_loop(grid: &mut Vec<Vec<GridPositionState>>) -> i32 {
    let mut number_of_possibilities = 0;
    let grid_size = grid.len();
    for i in 0..grid_size {
        for j in 0..grid_size {
            let state = &grid[i][j];
            if *state == GridPositionState::None {
                let mut cloned_grid = grid.clone();
                cloned_grid[i][j] = GridPositionState::Obstruction;
                if is_stuck_in_loop(&mut cloned_grid) {
                    number_of_possibilities += 1;
                }
            }
        }
    }

    number_of_possibilities
}
fn is_stuck_in_loop(grid: &mut Vec<Vec<GridPositionState>>) -> bool {
    let mut guard_outside_grid = false;
    let orientation_row: Vec<i32> = vec![0; grid.len()];
    let mut orientation_grid = vec![orientation_row; grid.len()];
    let (mut guard_i, mut guard_j, mut orientation) = find_guard(&grid);
    while !guard_outside_grid {
        if orientation_grid[guard_i][guard_j] == 5 { // check if guard passed by the same pos 5 times so it means she went at least from all directions once
            return true;
        }
        orientation_grid[guard_i][guard_j] += 1;
        if let Some((next_i, next_j)) = next_position(guard_i, guard_j, &orientation, grid.len()) {
            match grid[next_i][next_j] {
                GridPositionState::None => {
                    grid[guard_i][guard_j] = GridPositionState::Visited;
                    grid[next_i][next_j] = GridPositionState::Guard(orientation.clone());
                    guard_i = next_i;
                    guard_j = next_j;
                }
                GridPositionState::Visited => {
                    grid[guard_i][guard_j] = GridPositionState::Visited;
                    grid[next_i][next_j] = GridPositionState::Guard(orientation.clone());
                    guard_i = next_i;
                    guard_j = next_j;
                }
                GridPositionState::Obstacle | GridPositionState::Obstruction => {
                    let next_orientation = orientation.next_orientation();
                    grid[guard_i][guard_j] = GridPositionState::Guard(next_orientation);
                    orientation = next_orientation;
                }
                _ => panic!(), // Shouldn't happen
            }
        } else {
            guard_outside_grid = true;
            grid[guard_i][guard_j] = GridPositionState::Visited;
        }
    }

    false
}

#[derive(PartialEq, Debug, Clone)]
enum GridPositionState {
    None,
    Obstacle,
    Visited,
    Guard(Orientation),
    Obstruction,
}

#[derive(PartialEq, Clone, Debug, Copy)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn next_orientation(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
            Orientation::Right => Orientation::Down,
        }
    }
}

fn string_to_grid(text: String) -> Vec<Vec<GridPositionState>> {
    let mut grid = vec![];
    for s in text.split('\n') {
        let mut row = vec![];
        for c in s.chars() {
            match c {
                '#' => row.push(GridPositionState::Obstacle),
                '.' => row.push(GridPositionState::None),
                '^' => row.push(GridPositionState::Guard(Orientation::Up)),
                'O' => row.push(GridPositionState::Obstruction),
                _ => continue,
            }
        }
        grid.push(row);
    }

    grid
}

fn grid_to_string(grid: Vec<Vec<GridPositionState>>) -> String {
    let mut text = vec![];
    for row in grid {
        let mut line = vec![];
        for element in row {
            let c = match element {
                GridPositionState::None => '.',
                GridPositionState::Obstacle => '#',
                GridPositionState::Visited => 'X',
                GridPositionState::Guard(orientation) => match orientation {
                    Orientation::Up => '^',
                    Orientation::Down => '∨',
                    Orientation::Left => '<',
                    Orientation::Right => '>',
                },
                GridPositionState::Obstruction => 'O',
            };
            line.push(c);
        }
        text.push(line.iter().collect::<String>());
    }

    text.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_position_simple() {
        assert_eq!(next_position(3, 2, &Orientation::Up, 5), Some((2, 2)));
    }

    #[test]
    fn test_next_position_outside() {
        assert_eq!(next_position(4, 2, &Orientation::Down, 5), None);
    }

    #[test]
    fn test_string_to_grid() {
        let text = ".#.\n\
                          ...\n\
                          #^.";

        let expected_grid = vec![
            vec![
                GridPositionState::None,
                GridPositionState::Obstacle,
                GridPositionState::None,
            ],
            vec![
                GridPositionState::None,
                GridPositionState::None,
                GridPositionState::None,
            ],
            vec![
                GridPositionState::Obstacle,
                GridPositionState::Guard(Orientation::Up),
                GridPositionState::None,
            ],
        ];

        assert_eq!(string_to_grid(text.to_string()), expected_grid);
    }

    #[test]
    fn test_grid_to_string() {
        let grid = vec![
            vec![
                GridPositionState::None,
                GridPositionState::Obstacle,
                GridPositionState::None,
            ],
            vec![
                GridPositionState::None,
                GridPositionState::None,
                GridPositionState::None,
            ],
            vec![
                GridPositionState::Obstacle,
                GridPositionState::Guard(Orientation::Up),
                GridPositionState::None,
            ],
        ];

        let expected_text = ".#.\n\
                                   ...\n\
                                   #^.";

        assert_eq!(grid_to_string(grid), expected_text);
    }

    #[test]
    fn test_find_guard() {
        let text = ".#.\n\
                          ...\n\
                          #^.";

        let grid = string_to_grid(text.to_string());

        assert_eq!(find_guard(&grid), (2, 1, Orientation::Up));
    }

    #[test]
    fn test_predict_guard_route_simple() {
        let mut grid = vec![
            vec![
                GridPositionState::None,
                GridPositionState::Obstacle,
                GridPositionState::None,
            ],
            vec![
                GridPositionState::None,
                GridPositionState::None,
                GridPositionState::None,
            ],
            vec![
                GridPositionState::Obstacle,
                GridPositionState::Guard(Orientation::Up),
                GridPositionState::None,
            ],
        ];

        let expected_grid = vec![
            vec![
                GridPositionState::None,
                GridPositionState::Obstacle,
                GridPositionState::None,
            ],
            vec![
                GridPositionState::None,
                GridPositionState::Visited,
                GridPositionState::Visited,
            ],
            vec![
                GridPositionState::Obstacle,
                GridPositionState::Visited,
                GridPositionState::None,
            ],
        ];
        let visited_positions = predict_guard_route(&mut grid);
        assert_eq!(grid, expected_grid);
        assert_eq!(visited_positions, 3);
    }

    #[test]
    fn test_predict_guard_route() {
        let text = "....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...";
        let mut grid = string_to_grid(text.to_string());

        let expected_grid = "....#.....\n\
        ....XXXXX#\n\
        ....X...X.\n\
        ..#.X...X.\n\
        ..XXXXX#X.\n\
        ..X.X.X.X.\n\
        .#XXXXXXX.\n\
        .XXXXXXX#.\n\
        #XXXXXXX..\n\
        ......#X..";

        let visited_positions = predict_guard_route(&mut grid);
        assert_eq!(grid_to_string(grid), expected_grid);
        assert_eq!(visited_positions, 41);
    }

    #[test]
    fn test_is_stuck_in_loop_true() {
        let text = "....#.....\n\
         .........#\n\
         ..........\n\
         ..#.......\n\
         .......#..\n\
         ..........\n\
         .#.Oc^.....\n\
         ........#.\n\
         #.........\n\
         ......#...";
        let mut grid = string_to_grid(text.to_string());

        assert_eq!(is_stuck_in_loop(&mut grid), true);
    }

    #[test]
    fn test_is_stuck_in_loop_false() {
        let text = "....#.....\n\
         ....O....#\n\
         ..........\n\
         ..#.......\n\
         .......#..\n\
         ..........\n\
         .#..^.....\n\
         ........#.\n\
         #.........\n\
         ......#...";
        let mut grid = string_to_grid(text.to_string());

        assert_eq!(is_stuck_in_loop(&mut grid), false);
    }

    #[test]
    fn test_brute_force_possibilites_for_loop() {
        let text = "....#.....\n\
         .........#\n\
         ..........\n\
         ..#.......\n\
         .......#..\n\
         ..........\n\
         .#..^.....\n\
         ........#.\n\
         #.........\n\
         ......#...";
        let mut grid = string_to_grid(text.to_string());

        assert_eq!(brute_force_possibilites_for_loop(&mut grid), 6);
    }
}
