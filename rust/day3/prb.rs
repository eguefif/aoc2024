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
    let re = Regex::new(r"don't\(\)(.*)do\(\)").unwrap();

    let res = re.replace_all(input, "");

    println!("AFTER\n {}", res);
    part1(&res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exemple() {
        let ex = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let res = part1(ex);
        assert_eq!(161, res)
    }

    #[test]
    fn exemple2() {
        let ex = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let res = part2(ex);
        assert_eq!(48, res)
    }
}
