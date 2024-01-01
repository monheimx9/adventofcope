use std::{char, i64};

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    case: i64,
}

impl Instruction {
    fn from_str(s: &str) -> Vec<Instruction> {
        s.split(", ")
            .map(|st| Instruction {
                dir: Direction::from_char(st.trim().chars().nth(0).unwrap()),
                case: st[1..].trim().parse().unwrap(),
            })
            .collect()
    }
    fn case_num(&self) -> i64 {
        self.case
    }
    fn total(first: Cardinal, ins: &[Instruction]) -> i64 {
        let (mut x, mut y) = (0, 0);
        let mut x_tot: i64 = 0;
        let mut y_tot: i64 = 0;

        let mut curr_dir = first;
        for i in ins {
            (curr_dir, x, y) = i.dir.walk(curr_dir, i.case);
            x_tot += x;
            y_tot += y;
        }
        x_tot.abs() + y_tot.abs()
    }
}

enum Cardinal {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn from_char(s: char) -> Self {
        match s {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => unreachable!("Invalid direction"),
        }
    }
    fn walk(&self, dir: Cardinal, case: i64) -> (Cardinal, i64, i64) {
        match dir {
            Cardinal::North => match self {
                Self::Right => (Cardinal::East, case, 0),
                Self::Left => (Cardinal::West, case * -1, 0),
            },
            Cardinal::South => match self {
                Self::Right => (Cardinal::West, case * -1, 0),
                Self::Left => (Cardinal::East, case, 0),
            },
            Cardinal::West => match self {
                Self::Right => (Cardinal::North, 0, case),
                Self::Left => (Cardinal::South, 0, case * -1),
            },
            Cardinal::East => match self {
                Self::Right => (Cardinal::South, 0, case * -1),
                Self::Left => (Cardinal::North, 0, case),
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
    let s = include_str!("../input.txt");
    let d = Instruction::from_str(s);
    let ans = Instruction::total(Cardinal::North, &d);
    println!("Part One = {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_1() {
        let s = "R5, L5, R5, R3";
        let d = Instruction::from_str(s);
        let ans = Instruction::total(Cardinal::North, &d);
        assert_eq!(ans, 12)
    }
    #[test]
    fn pt1_2() {
        let s = "R2, L3";
        let d = Instruction::from_str(s);
        let ans = Instruction::total(Cardinal::North, &d);
        assert_eq!(ans, 5)
    }
    #[test]
    fn pt1_3() {
        let s = "R2, R2, R2";
        let d = Instruction::from_str(s);
        let ans = Instruction::total(Cardinal::North, &d);
        assert_eq!(ans, 2)
    }
}
