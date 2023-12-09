use core::panic;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

use num::integer::lcm;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left(Option<String>),
    Right(Option<String>),
}
impl Direction {
    fn from_char(c: &char) -> Self {
        match c {
            'R' => Direction::Right(None),
            'L' => Direction::Left(None),
            _ => panic!("Invalid character"),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Node {
    dirs: HashSet<Direction>,
}
impl Node {
    fn new(r: &str, l: &str) -> Self {
        let ri = Direction::Right(Some(r.to_string()));
        let le = Direction::Left(Some(l.to_string()));
        let mut dirs: HashSet<Direction> = HashSet::new();
        dirs.insert(ri);
        dirs.insert(le);
        Node { dirs }
    }
    fn get(&self, d: &Direction) -> Option<String> {
        for dir in &self.dirs {
            match (dir, d) {
                (Direction::Right(_), Direction::Right(_))
                | (Direction::Left(_), Direction::Left(_)) => {
                    if let Some(info) = match dir {
                        Direction::Right(info) => info,
                        Direction::Left(info) => info,
                    } {
                        return Some(info.clone());
                    }
                }
                _ => continue,
            }
        }
        None
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Instructions(Vec<Direction>);

impl Instructions {
    fn from_str(s: &str) -> Self {
        Instructions(s.chars().map(|c| Direction::from_char(&c)).collect())
    }
}
struct Network(HashMap<String, Node>, Instructions);

impl Network {
    fn from_str(s: &str) -> Self {
        let ss = s.lines().collect::<Vec<&str>>();
        let ins = Instructions::from_str(ss[0]);
        Network(
            ss.iter()
                .skip(1)
                .filter(|l| l.len() > 0)
                .map(|f| Network::parse_node(f))
                .collect(),
            ins,
        )
    }
    fn parse_node(s: &str) -> (String, Node) {
        let re = regex::Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
        let re = re.captures(s).unwrap();
        let (k, l, r) = (re.get(1).unwrap(), re.get(2).unwrap(), re.get(3).unwrap());
        let node = Node::new(r.as_str(), l.as_str());
        (k.as_str().to_string(), node)
    }
    fn travel(&self, start: String, end: String) -> usize {
        let ins = &self.1 .0;
        let mut count: usize = 0;
        let mut start = start.clone();
        for n in ins.iter().cycle() {
            count += 1;
            let next = self.0.get(&start).unwrap();
            start = next.get(n).unwrap();
            if start.trim() == end.trim() {
                break;
            }
        }
        count
    }
    fn ghost_travel(&self) -> usize {
        let ins = &self.1 .0;
        let mut count: usize = 0;
        let start_vec = self
            .0
            .iter()
            .filter(|c| c.0.ends_with('A'))
            .map(|h| h.0.to_string())
            .collect::<Vec<String>>();
        let mut end_nodes = start_vec.clone();

        // 'outer: while true {
        //     for dir in ins.iter().cycle() {
        //         count += 1;
        //         for n in 0..end_nodes.len() {
        //             let next = self.0.get(&end_nodes[n]).unwrap();
        //             end_nodes[n] = next.get(dir).unwrap().clone();
        //         }
        //         if end_nodes.iter().all(|f| f.ends_with('Z')) {
        //             break 'outer;
        //         }
        //     }
        // }

        let mut step_vec: Vec<usize> = Vec::new();
        for start in start_vec {
            let mut new_start = start.clone();
            for (i, n) in ins.iter().cycle().enumerate() {
                count += 1;
                let next = self.0.get(&new_start).unwrap();
                new_start = next.get(n).unwrap();
                if new_start.trim().ends_with('Z') {
                    step_vec.push(i + 1);
                    let a = i + 1;
                    println!("Start: {start} | End: {new_start} | Steps at {a}");
                    break;
                }
            }
        }
        step_vec.iter().fold(1, |acc, &x| lcm(acc, x))
        // count
    }
}

fn main() {
    println!("Hello, world!");
    let i = include_str!("../input.txt");
    let n = Network::from_str(i).travel("AAA".to_string(), "ZZZ".to_string());
    println!("Part one: {n}");
    let n = Network::from_str(i).ghost_travel();
    println!("Part two: {n}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_1() {
        let i = include_str!("../test1.txt");
        let n = Network::from_str(i).travel("AAA".to_string(), "ZZZ".to_string());
        assert_eq!(n, 2)
    }
    #[test]
    fn pt1_2() {
        let i = include_str!("../test2.txt");
        let n = Network::from_str(i).travel("AAA".to_string(), "ZZZ".to_string());
        assert_eq!(n, 6)
    }
    #[test]
    fn pt2_1() {
        let i = include_str!("../test2.txt");
        let n = Network::from_str(i).ghost_travel();
        assert_eq!(n, 6)
    }
}
