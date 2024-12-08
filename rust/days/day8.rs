use std::collections::HashSet;

fn main() {
    let grid = get_input();
    print_grid(&grid);
    println!("Part2: {}", part2(grid.clone()));
}

fn part2(grid: Vec<Vec<char>>) -> usize {
    let mut points: HashSet<(usize, usize)> = HashSet::new();

    let g = grid.clone();
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if is_antinode(x, y, g.clone()) {
                points.insert((x, y));
            }
        }
    }

    points.len()
}

fn is_antinode(x: usize, y: usize, grid: Vec<Vec<char>>) -> bool {
    for (ya, row) in grid.iter().enumerate() {
        for (xa, _) in row.iter().enumerate() {
            if check(x, y, xa, ya, grid.clone()) {
                return true;
            }
        }
    }
    return false;
}

fn check(x: usize, y: usize, xa: usize, ya: usize, grid: Vec<Vec<char>>) -> bool {
    let mut vertical_line = false;
    let slope;
    if x == xa {
        vertical_line = true;
        slope = 0;
    } else {
        slope = (ya as i32 - y as i32) / (xa as i32 - x as i32);
    }
    let intercept = ya as i32 - xa as i32 * slope;

    //println!("p1: {} {}, p2 {} {}", x, y, xa, ya);
    //println!("slope: {}, intercept {}", slope, intercept);
    let sym = grid[ya][xa];
    for (ya2, row) in grid.iter().enumerate() {
        for (xa2, _) in row.iter().enumerate() {
            if (grid[ya2][xa2] != sym) || (ya == ya2 && xa == xa2) {
                continue;
            }
            if is_on_line(
                xa2 as i32,
                ya2 as i32,
                slope,
                intercept,
                vertical_line,
                x as i32,
            ) {
                return true;
            }
        }
    }
    false
}

fn is_in_grid(x: usize, y: usize, grid: Vec<Vec<char>>) -> bool {
    x < grid[0].len() && y < grid.len()
}

fn is_on_line(
    x: i32,
    y: i32,
    slope: i32,
    intercept: i32,
    vertical_line: bool,
    initial_x: i32,
) -> bool {
    if vertical_line && initial_x == x {
        return true;
    }
    y == x * slope + intercept
}
fn print_grid(grid: &[Vec<char>]) {
    for row in grid.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn get_input() -> Vec<Vec<char>> {
    let content = include_str!("../../inputs/exemple");
    content.lines().map(|row| row.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_d8_part2() {
        let grid = get_input();
        let res = part2(grid.clone());

        assert_eq!(res, 34)
    }
}
