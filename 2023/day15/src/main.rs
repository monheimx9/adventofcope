use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::usize;

struct LensBox(HashMap<usize, LensLabel>);

struct LensLabel(Vec<(String, usize)>);

fn main() {
    println!("Hello, world!");
}

fn solve_hash(sl: &[&str]) -> usize {
    sl.iter()
        .map(|s| {
            s.trim()
                .chars()
                .filter(|i| i.is_ascii())
                .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
        })
        .sum()
}

fn hash_label(s: &str) -> usize {
    s.trim()
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn solve_box(sl: &[&str]) -> usize {
    let mut lbox: LensBox = LensBox(HashMap::new());

    for s in sl.iter() {
        if let Some((h, l)) = s.split_once('=') {
            let ha = hash_label(h);
            let le: usize = l.parse().unwrap();
            perform_stuff_in_box(&mut lbox, &Ope::Replace, h, ha, le);
        }
    }
}

enum Ope {
    Replace,
    Remove,
}

fn perform_stuff_in_box(b: &mut LensBox, op: &Ope, labstr: &str, labhash: usize, lens: usize) {
    match op {
        Ope::Replace => {}
        Ope::Remove => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_1() {
        let s = include_str!("../test.txt")
            .trim()
            .split(',')
            .collect::<Vec<_>>();
        let o = solve_hash(&s);
        assert_eq!(o, 1320)
    }
    #[test]
    fn pt1_2() {
        let s = vec!["HASH"];
        let o = solve_hash(&s);
        assert_eq!(o, 52)
    }
    #[test]
    fn pt1_3() {
        let s = vec!["rn"];
        let o = solve_hash(&s);
        let s1 = vec!["cm"];
        let o2 = solve_hash(&s1);
        assert_eq!(o, o2)
    }
    #[test]
    fn pt1() {
        let s = include_str!("../input.txt")
            .trim()
            .split(',')
            .collect::<Vec<_>>();
        let o = solve_hash(&s);
        assert_eq!(o, 513158)
    }
}
