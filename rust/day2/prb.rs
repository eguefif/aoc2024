fn main() {
    let input = get_input();
    println!("{:?}", part1(input.clone()));
    println!("{:?}", part2(input.clone()));
}

fn part1(input: Vec<Vec<i32>>) -> i32 {
    input
        .into_iter()
        .filter(|row| is_row_safe_p1(row))
        .collect::<Vec<_>>()
        .len() as i32
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

fn is_row_safe_p1(row: &Vec<i32>) -> bool {
    (row.iter().is_sorted() || row.iter().rev().is_sorted()) && check(&row) && row.len() > 0
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

    let mut retval = vec![];
    content.lines().for_each(|line| {
        let mut v = vec![];
        let split = line.split_ascii_whitespace();
        for s in split {
            v.push(s.parse::<i32>().unwrap());
        }
        retval.push(v);
    });

    retval
}
