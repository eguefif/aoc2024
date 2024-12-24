#[derive(Copy, Clone)]
struct Robot {
    pub x: usize,
    pub y: usize,
}

impl Robot {
    pub fn new(map: Vec<Vec<char>>) -> Option<Robot> {
        for (y, row) in map.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                if *v == '@' {
                    return Some(Robot { x, y });
                }
            }
        }
        None
    }

    pub fn move_robot(&mut self, coord: &(usize, usize)) {
        self.x = coord.0;
        self.y = coord.1;
    }
}

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input.0.clone(), input.1.clone()));
    let (map, moves) = get_input_part2();
    let robot = Robot::new(map.clone());
    dump(map.clone(), robot.unwrap());
    println!("Part2: {}", part2(map.clone(), moves.clone()));
}

fn part2(mut map: Vec<Vec<char>>, moves: String) -> usize {
    if let Some(mut robot) = Robot::new(map.clone()) {
        for m in moves.chars() {
            let next = get_next_coordinates(&(robot.x, robot.y), m);
            if map[next.1][next.0] == '.' || map[next.1][next.0] == '@' {
                robot.move_robot(&next);
            } else if map[next.1][next.0] == '[' || map[next.1][next.0] == ']' {
                if move_crate2(&next, &mut map, m, 0) {
                    robot.move_robot(&next);
                }
            }
        }
    }
    return calculate_coordinates(map);
}

fn part1(mut map: Vec<Vec<char>>, moves: String) -> usize {
    if let Some(mut robot) = Robot::new(map.clone()) {
        for m in moves.chars() {
            let next = get_next_coordinates(&(robot.x, robot.y), m);
            if map[next.1][next.0] == '.' || map[next.1][next.0] == '@' {
                robot.move_robot(&next);
            } else if map[next.1][next.0] == 'O' {
                if move_crate(&next, &mut map, m, 0) {
                    robot.move_robot(&next);
                }
            }
        }
    }
    return calculate_coordinates(map);
}

fn move_crate(next: &(usize, usize), map: &mut Vec<Vec<char>>, m: char, rec: i32) -> bool {
    let n = get_next_coordinates(next, m);
    if map[n.1][n.0] == '#' {
        return false;
    }
    if map[n.1][n.0] == '.' || map[n.1][n.0] == '@' {
        map[next.1][next.0] = '.';
        map[n.1][n.0] = 'O';
        return true;
    } else if map[n.1][n.0] == 'O' {
        if move_crate(&n, map, m, rec + 1) {
            map[next.1][next.0] = '.';
            map[n.1][n.0] = 'O';
            return true;
        }
    }
    false
}

fn move_crate2(next: &(usize, usize), map: &mut Vec<Vec<char>>, m: char, rec: i32) -> bool {
    if m == '<' || m == '>' {
        move_crate(next, map, m, rec);
    }
    let n = get_next_coordinates(next, m);
    if map[n.1][n.0] == '#' {
        return false;
    }
    if map[n.1][n.0] == '.' || map[n.1][n.0] == '@' {
        map[next.1][next.0] = '.';
        map[n.1][n.0] = 'O';
        return true;
    } else if map[n.1][n.0] == 'O' {
        if move_crate(&n, map, m, rec + 1) {
            map[next.1][next.0] = '.';
            map[n.1][n.0] = 'O';
            return true;
        }
    }
    false
}

fn get_next_coordinates(coord: &(usize, usize), m: char) -> (usize, usize) {
    match m {
        '<' => (coord.0 - 1, coord.1),
        '>' => (coord.0 + 1, coord.1),
        '^' => (coord.0, coord.1 - 1),
        'v' => (coord.0, coord.1 + 1),
        _ => panic!("Wrong move"),
    }
}

fn calculate_coordinates(map: Vec<Vec<char>>) -> usize {
    let mut acc: usize = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if *v == 'O' {
                acc += 100 * y + x;
            }
        }
    }
    acc
}

fn dump(map: Vec<Vec<char>>, robot: Robot) {
    println!("Robot x: {}, y: {}", robot.x, robot.y);
    for (y, row) in map.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if x == robot.x && y == robot.y {
                print!("@");
            } else {
                if *v == '@' {
                    print!(".");
                } else {
                    print!("{}", *v);
                }
            }
        }
        println!("");
    }
    println!("");
}

fn get_input() -> (Vec<Vec<char>>, String) {
    let content = include_str!("../../inputs/exemple");
    let mut splits = content.split("\n\n");

    let raw_map = splits.next().unwrap();
    let raw_moves = splits.next().unwrap();

    let moves = raw_moves.lines().fold(String::new(), |mut retval, line| {
        retval.push_str(line);
        retval
    });

    let map = raw_map.lines().map(|line| line.chars().collect()).collect();

    (map, moves)
}

fn get_input_part2() -> (Vec<Vec<char>>, String) {
    let input_p1 = get_input();
    let mut map: Vec<Vec<char>> = vec![];

    for row in input_p1.0.iter() {
        let mut new_row: Vec<char> = vec![];
        for c in row.iter() {
            if *c == '#' {
                new_row.push('#');
                new_row.push('#');
            } else if *c == 'O' {
                new_row.push('[');
                new_row.push(']');
            } else if *c == '.' {
                new_row.push('.');
                new_row.push('.');
            } else if *c == '@' {
                new_row.push('@');
                new_row.push('.');
            }
        }
        map.push(new_row);
    }

    (map, input_p1.1)
}
