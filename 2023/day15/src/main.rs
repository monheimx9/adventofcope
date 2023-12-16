use std::collections::BTreeMap;
use std::time::Instant;
use std::usize;

struct LensBox(BTreeMap<usize, LensLabel>);

struct LensLabel(Vec<(String, usize)>);

fn main() {
    println!("Hello, world!");
    let s = include_str!("../input.txt")
        .trim()
        .split(',')
        .collect::<Vec<_>>();
    let ins = Instant::now();
    let o = solve_hash(&s);
    println!("Part one is {}", o);
    println!("In: {} micros", ins.elapsed().as_micros());
    let ins = Instant::now();
    let o = solve_box(&s);
    println!("Part two is {}", o);
    println!("In: {} micros", ins.elapsed().as_micros());
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
    let mut lbox: LensBox = LensBox(BTreeMap::new());

    for s in sl.iter() {
        if let Some((h, l)) = s.split_once('=') {
            let ha = hash_label(h);
            let le: usize = l.parse().unwrap();
            perform_stuff_in_box(&mut lbox, &Ope::Replace, h, ha, le);
        }
        if let Some((h, _)) = s.split_once('-') {
            let ha = hash_label(h);
            perform_stuff_in_box(&mut lbox, &Ope::Remove, h, ha, 0);
        }
    }
    lbox.0
        .iter()
        .enumerate()
        .map(|(_, (i, b))| {
            b.0.iter()
                .enumerate()
                .map(|(i2, (_, l))| (i + 1) * (i2 + 1) * l)
                .sum::<usize>()
        })
        .sum()
}

enum Ope {
    Replace,
    Remove,
}

fn perform_stuff_in_box(b: &mut LensBox, op: &Ope, labstr: &str, labhash: usize, lens: usize) {
    match op {
        Ope::Replace => {
            let bo = b.0.entry(labhash).or_insert(LensLabel(Vec::new()));
            if bo.0.is_empty() {
                bo.0.push((labstr.to_string(), lens));
            } else {
                let mut found = false;
                for l in 0..bo.0.len() {
                    if bo.0[l].0 == labstr {
                        bo.0[l].1 = lens;
                        found = true;
                    }
                }
                if !found {
                    bo.0.push((labstr.to_string(), lens));
                }
            }
        }
        Ope::Remove => {
            if let Some(bo) = b.0.get_mut(&labhash) {
                for l in 0..bo.0.len() {
                    if bo.0[l].0 == labstr {
                        bo.0.remove(l);
                        break;
                    }
                }
            }
        }
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
    #[test]
    fn pt2_1() {
        let s = include_str!("../test.txt")
            .trim()
            .split(',')
            .collect::<Vec<_>>();
        let o = solve_box(&s);
        assert_eq!(o, 145)
    }
    #[test]
    fn pt2() {
        let s = include_str!("../input.txt")
            .trim()
            .split(',')
            .collect::<Vec<_>>();
        let o = solve_box(&s);
        assert_eq!(o, 200277)
    }
}
