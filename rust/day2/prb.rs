fn main() {
    let input = get_input();
    println!("{:?}", part1(input.clone()));
    println!("{:?}", part2(input.clone()));
}

fn part1(input: Vec<Vec<i32>>) -> usize {
    input.into_iter().filter(|row| is_row_safe_p1(row)).count()
}

fn is_row_safe_p1(row: &Vec<i32>) -> bool {
    let d = row
        .iter()
        .zip(row.iter().skip(1))
        .map(|(a, b)| a - b)
        .collect::<Vec<i31>>();

    d.iter().all(|x| *x >= 1 && *x <= 3) || d.iter().all(|x| *x <= -1 && *x >= -3)
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    input
        .into_iter()
        .filter(|row| is_row_safe(row))
        .collect::<Vec<_>>()
        .len() as i32
}

fn is_row_safe(roww: &Vec<i32>) -> bool {
    let n = roww.len();
    for x in 0..n {
        let mut row = roww.clone();
        row.remove(x);
        if (row.iter().is_sorted() || row.iter().rev().is_sorted()) && check(&row) && row.len() > 0
        {
            return true;
        }
    }
    return false;
}

fn check(row: &Vec<i32>) -> bool {
    for (i, a) in row.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let r = (a - row[i - 1]).abs();
        if r < 1 || r > 3 {
            return false;
        }
    }
    true
}

fn get_input() -> Vec<Vec<i32>> {
    let content = include_str!("../../inputs/d2");
    //let content = include_str!("./exemple");

    content
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|item| item.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_d2p1() {
        let input = get_input();
        let res = part1(input);
        assert_eq!(299, res)
    }

    #[test]
    fn it_d2p2() {
        let input = get_input();
        let res = part2(input);
        assert_eq!(364, res)
    }
}
