fn main() {
    let mut input = get_input();
    println!("part2: {}", part2(&mut input));
}

fn part2(input: &mut Vec<Vec<char>>) -> u32 {
    let mut guard = get_guard(input);
    let mut direction = (0, -1);
    while !going_outside(guard, input, direction) {
        if can_move(guard, input, direction) {
            guard = walk(guard, direction);
            input[guard.1 as usize][guard.0 as usize] = 'x'
        } else {
            direction = turn(direction);
        }
    }
    print_input(input);
    count_x(input)
}

fn print_input(input: &mut Vec<Vec<char>>) {
    for row in input.iter() {
        println!("{:?}", row);
    }
}

fn count_x(input: &mut Vec<Vec<char>>) -> u32 {
    input.iter().fold(0, |mut acc, row| {
        acc += row.iter().filter(|x| **x == 'x').count() as u32;
        acc
    })
}

fn get_guard(input: &mut Vec<Vec<char>>) -> (i32, i32) {
    for (y, row) in input.iter().enumerate() {
        for (x, loc) in row.iter().enumerate() {
            if *loc == '^' {
                return (x as i32, y as i32);
            }
        }
    }
    return (0, 0);
}

fn going_outside(guard: (i32, i32), input: &mut Vec<Vec<char>>, direction: (i32, i32)) -> bool {
    let (x, y) = (guard.0 + direction.0, guard.1 + direction.1);
    if let Some(row) = input.get(y as usize) {
        if let Some(_) = row.get(x as usize) {
            return false;
        }
    }
    return true;
}

fn can_move(guard: (i32, i32), input: &mut Vec<Vec<char>>, direction: (i32, i32)) -> bool {
    let (x, y) = (guard.0 + direction.0, guard.1 + direction.1);
    if input[y as usize][x as usize] == '#' {
        return false;
    }
    return true;
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
    let content = include_str!("../../inputs/exemple");
    content.lines().fold(Vec::new(), |mut input, row| {
        input.push(row.chars().collect());
        input
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn it_part2() {
        let mut input = get_input();
        let res = part2(&mut input);
        assert_eq!(res, 6)
    }
}
