use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/d3");
    let res = part1(input);
    let res2 = part2(input);
    println!("part1: {res}");
    println!("part2: {res2}");
}

fn part1(input: &str) -> usize {
    let reg = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut acc = 0;
    for (_, [n1, n2]) in reg.captures_iter(input).map(|x| x.extract()) {
        acc += n1.parse::<usize>().unwrap() * n2.parse::<usize>().unwrap();
    }

    acc
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"(don't\(\))|(mul\([0-9]+,[0-9]+\))|(do\(\))").unwrap();

    let mut acc = 0;
    let mut flag = true;
    for (token, [_]) in re.captures_iter(input).map(|v| v.extract()) {
        if token.matches("don't()").collect::<Vec<&str>>().len() > 0 {
            flag = false;
        } else if token.matches("do()").collect::<Vec<&str>>().len() > 0 {
            flag = true;
        } else if flag == true && token.matches("mul").collect::<Vec<&str>>().len() > 0 {
            acc += part1(token);
        }
    }

    acc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exemple() {
        let input = include_str!("../../inputs/d3");
        let res = part1(input);
        assert_eq!(163931492, res)
    }

    #[test]
    fn exemple2() {
        let input = include_str!("../../inputs/d3");
        let res = part2(input);
        assert_eq!(76911921, res)
    }
}
