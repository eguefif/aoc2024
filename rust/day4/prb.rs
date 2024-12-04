fn main() {
    let input = get_input();
    let res = part1(input.clone());
    let res2 = part2(input.clone());
    println!("part1: {res}");
    println!("part2: {res2}");
}

fn part2(input: Vec<Vec<char>>) -> usize {
    let height = input.len();
    let width = input[0].len();

    let mut acc = 0;
    for y in 0..height {
        for x in 0..width {
            let c = input[y][x];
            if c == 'S' || c == 'M' {
                acc += check2(&input, y, x)
            }
        }
    }
    acc
}

fn check2(input: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let height = input.len();
    let width = input[0].len();
    let mut acc = 0;

    if x + 3 < width && y + 3 < height {
        let mut tmp1 = String::new();
        tmp1.push(input[y][x]);
        tmp1.push(input[y + 1][x + 1]);
        tmp1.push(input[y + 2][x + 2]);

        let mut tmp2 = String::new();
        tmp2.push(input[y + 2][x]);
        tmp2.push(input[y + 1][x + 1]);
        tmp2.push(input[y][x + 2]);
        if check_word(&tmp1, &tmp2) {
            acc += 1;
        }
    }

    acc
}

fn check_word(w1: &str, w2: &str) -> bool {
    (w1 == "SAM" || w1 == "MAS") && (w2 == "SAM" || w2 == "MAS")
}

fn part1(input: Vec<Vec<char>>) -> usize {
    let height = input.len();
    let width = input[0].len();

    let mut acc = 0;
    for y in 0..height {
        for x in 0..width {
            if input[y][x] == 'X' {
                acc += check1(&input, y, x)
            }
        }
    }
    acc
}

fn check1(input: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let height = input.len();
    let width = input[0].len();
    let mut acc = 0;
    let xmas = "XMAS";

    if x + 3 < width {
        let mut tmp = String::new();
        tmp.push(input[y][x]);
        tmp.push(input[y][x + 1]);
        tmp.push(input[y][x + 2]);
        tmp.push(input[y][x + 3]);
        if tmp.as_str() == xmas {
            acc += 1;
        }
    }

    if x.wrapping_sub(3) < width {
        let mut tmp = String::new();
        tmp.push(input[y][x]);
        tmp.push(input[y][x - 1]);
        tmp.push(input[y][x - 2]);
        tmp.push(input[y][x - 3]);
        if tmp.as_str() == xmas {
            acc += 1;
        }
    }

    if y.wrapping_sub(3) < height {
        let mut tmp = String::new();
        tmp.push(input[y][x]);
        tmp.push(input[y - 1][x]);
        tmp.push(input[y - 2][x]);
        tmp.push(input[y - 3][x]);
        if tmp.as_str() == xmas {
            acc += 1;
        }
    }

    if y + 3 < height {
        let mut tmp = String::new();
        tmp.push(input[y][x]);
        tmp.push(input[y + 1][x]);
        tmp.push(input[y + 2][x]);
        tmp.push(input[y + 3][x]);
        if tmp.as_str() == xmas {
            acc += 1;
        }
    }

    if y + 3 < height && x + 3 < width {
        let mut tmp = String::new();
        tmp.push(input[y][x]);
        tmp.push(input[y + 1][x + 1]);
        tmp.push(input[y + 2][x + 2]);
        tmp.push(input[y + 3][x + 3]);
        if tmp.as_str() == xmas {
            acc += 1;
        }
    }

    if y.wrapping_sub(3) < height && x + 3 < width {
        let mut tmp = String::new();
        tmp.push(input[y][x]);
        tmp.push(input[y - 1][x + 1]);
        tmp.push(input[y - 2][x + 2]);
        tmp.push(input[y - 3][x + 3]);
        if tmp.as_str() == xmas {
            acc += 1;
        }
    }

    if y.wrapping_sub(3) < height && x.wrapping_sub(3) < width {
        let mut tmp = String::new();
        tmp.push(input[y][x]);
        tmp.push(input[y - 1][x - 1]);
        tmp.push(input[y - 2][x - 2]);
        tmp.push(input[y - 3][x - 3]);
        if tmp.as_str() == xmas {
            acc += 1;
        }
    }

    if y + 3 < height && x.wrapping_sub(3) < width {
        let mut tmp = String::new();
        tmp.push(input[y][x]);
        tmp.push(input[y + 1][x - 1]);
        tmp.push(input[y + 2][x - 2]);
        tmp.push(input[y + 3][x - 3]);
        if tmp.as_str() == xmas {
            acc += 1;
        }
    }
    acc
}

fn get_input() -> Vec<Vec<char>> {
    let data = include_str!("../../inputs/d4");
    //let data = include_str!("../../inputs/exemple");
    data.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exemple() {
        let input = get_input();
        let res = part1(input);
        assert_eq!(18, res)
    }

    #[test]
    fn exemple2() {
        let input = get_input();
        let res = part2(input);
        assert_eq!(9, res)
    }
}
