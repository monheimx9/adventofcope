#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    println!("Hello, world!");
    let s = include_str!("../input.txt");
    assert_eq!(part1(s), 69883);
    assert_eq!(part2(s), 207576);
}
fn part1(s: &str) -> usize {
    let newstring = s.split_blank();
    count_max(&newstring)
}
fn part2(s: &str) -> usize {
    let newstring = s.split_blank();
    top(&newstring)
}

fn count_max(v: &[&str]) -> usize {
    v.iter()
        .map(|l| l.lines().map(|s| s.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

fn top(v: &[&str]) -> usize {
    let mut list: Vec<usize> = v
        .iter()
        .map(|l| l.lines().map(|s| s.parse::<usize>().unwrap()).sum())
        .collect();
    list.sort_by(|a, b| b.cmp(a));
    list.iter().take(3).sum()
}

trait SplitBlank {
    fn split_blank(&self) -> Vec<&str>;
}

impl SplitBlank for str {
    fn split_blank(&self) -> Vec<&str> {
        self.split("\n\n").collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_1() {
        let teststr = include_str!("../sample.txt");
        assert_eq!(part1(teststr), 24000);
    }
}
