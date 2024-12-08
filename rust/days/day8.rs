use std::{collections::HashSet, usize};

fn main() {
    println!("Part2: {}", part2());
}

fn part2() -> usize {
    let (width, height) = get_input_grid_sizes();
    let mut points: HashSet<(i32, i32)> = HashSet::new();

    let pairs = get_uniq_pairs(width);
    let max_iterations = height.max(width);

    for pair in pairs.iter() {
        let p1 = pair.0;
        let p2 = pair.1;
        points.insert(p1);
        points.insert(p2);
        let slope = (p2.0 - p1.0, p2.1 - p1.1);

        for i in 1..max_iterations {
            let x = p1.0 - i * slope.0;
            let y = p1.1 - i * slope.1;
            if !is_in_grid(x, y, width, height) {
                break;
            }
            points.insert((x, y));
        }

        for i in 1..max_iterations {
            let x = p2.0 + i * slope.0;
            let y = p2.1 + i * slope.1;
            if !is_in_grid(x, y, width, height) {
                break;
            }
            points.insert((x, y));
        }
    }
    points.len()
}

fn get_uniq_pairs(width: i32) -> HashSet<((i32, i32), (i32, i32))> {
    let mut retval: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let input = get_input_array();

    for (i, v) in input.iter().enumerate() {
        if *v == '.' {
            continue;
        }
        let p1 = (i as i32 % width, (i as i32 / width) as i32);
        for (i2, v2) in input.iter().skip(i).enumerate() {
            let idx = i2 as i32 + i as i32;
            let p2 = (idx % width, (idx / width) as i32);
            if *v == *v2 && p1 != p2 {
                retval.insert((p1, p2));
            }
        }
    }
    retval
}

fn is_in_grid(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

fn get_input_array() -> Vec<char> {
    let content = include_str!("../../inputs/d8");
    content.chars().filter(|v| *v != '\n').collect()
}

fn get_input_grid_sizes() -> (i32, i32) {
    let content = include_str!("../../inputs/d8");
    let grid = content.lines().collect::<Vec<&str>>();
    (grid[0].len() as i32, grid.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_d8_part2() {
        let res = part2();

        assert_eq!(res, 1150)
    }
}
