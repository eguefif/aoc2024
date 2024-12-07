use std::collections::HashSet;

fn main() {
    let input = get_input();
    println!("part2: {}", part2(input.clone()));
}

fn part2(input: Vec<Vec<char>>) -> u32 {
    let guard = get_guard(&input);
    let mut acc = 0;
    let visited = get_visited(&input);

    for loc in visited.iter() {
        if loc.0 == guard.0 && loc.1 == guard.1 {
            continue;
        }
        if check(*loc, input.clone()) {
            acc += 1
        }
    }
    acc
}

fn get_visited(input: &[Vec<char>]) -> HashSet<(i32, i32)> {
    let mut guard = get_guard(input);
    let mut direction = (0, -1);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    while !going_outside(guard, input, direction) {
        if can_move(guard, input, direction) {
            guard = walk(guard, direction);
            visited.insert(guard);
        } else {
            direction = turn(direction);
        }
    }
    visited
}

fn check(loc: (i32, i32), input: Vec<Vec<char>>) -> bool {
    let mut history: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut guard = get_guard(&input);
    let mut direction = (0, -1);

    let mut potential = input.clone();
    potential[loc.1 as usize][loc.0 as usize] = '#';

    while !going_outside(guard, &potential, direction) {
        if history.contains(&(guard, direction)) {
            return true;
        }
        history.insert((guard, direction));
        if can_move(guard, &potential, direction) {
            guard = walk(guard, direction);
        } else {
            direction = turn(direction);
        }
    }
    false
}

#[allow(dead_code)]
fn print_input(input: &[Vec<char>], visited: &HashSet<(i32, i32)>) {
    for (y, row) in input.iter().enumerate() {
        for (x, i) in row.iter().enumerate() {
            if visited.contains(&(x as i32, y as i32)) {
                print!("x");
            } else {
                print!("{}", i);
            }
        }
        println!();
    }
}

fn get_guard(input: &[Vec<char>]) -> (i32, i32) {
    for (y, row) in input.iter().enumerate() {
        for (x, loc) in row.iter().enumerate() {
            if *loc == '^' {
                return (x as i32, y as i32);
            }
        }
    }
    (0, 0)
}

fn going_outside(guard: (i32, i32), input: &[Vec<char>], direction: (i32, i32)) -> bool {
    let (x, y) = (guard.0 + direction.0, guard.1 + direction.1);
    x < 0 || y < 0 || x >= input[0].len() as i32 || y >= input.len() as i32
}

fn can_move(guard: (i32, i32), input: &[Vec<char>], direction: (i32, i32)) -> bool {
    let (x, y) = (guard.0 + direction.0, guard.1 + direction.1);
    if input[y as usize][x as usize] == '#' {
        return false;
    }
    true
}

fn walk(guard: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    (guard.0 + direction.0, guard.1 + direction.1)
}

fn turn(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => (1, 0),
    }
}

fn get_input() -> Vec<Vec<char>> {
    let content = include_str!("../../inputs/d6");
    content.lines().fold(Vec::new(), |mut input, row| {
        input.push(row.chars().collect());
        input
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_part2() {
        let input = get_input();
        let res = part2(input.clone());
        assert_eq!(res, 1586)
    }
}
