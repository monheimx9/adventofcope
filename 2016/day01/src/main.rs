use std::collections::HashSet;

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
    fn first_double_visit(first: Cardinal, ins: &[Instruction]) -> i64 {
        let (mut x, mut y) = (0, 0);
        let mut double: HashSet<(i64, i64)> = HashSet::new();
        let mut x_tot: i64 = 0;
        let mut y_tot: i64 = 0;
        let mut curr_dir = first;
        let mut is_visited = false;
        let mut ans: i64 = 0;
        for i in ins {
            (curr_dir, x, y) = i.dir.walk(curr_dir, i.case);
            if x != 0 {
                (is_visited, ans) = walk_x(&mut double, x_tot, x, y_tot);
            } else {
                (is_visited, ans) = walk_y(&mut double, y_tot, y, x_tot);
            }
            if is_visited {
                return ans;
            }
            x_tot += x;
            y_tot += y;
        }
        return 0;
    }
}

fn walk_x(d: &mut HashSet<(i64, i64)>, x_tot: i64, x: i64, y_tot: i64) -> (bool, i64) {
    let mut x_t: i64 = x_tot;
    if x < 0 {
        for n in 1..=x.abs() {
            x_t += -1;
            if d.contains(&(x_t, y_tot)) {
                return (true, x_t.abs() + y_tot.abs());
            }
            d.insert((x_t, y_tot));
        }
    } else {
        for n in 1..=x.abs() {
            x_t += 1;
            if d.contains(&(x_t, y_tot)) {
                return (true, x_t.abs() + y_tot.abs());
            }
            d.insert((x_t, y_tot));
        }
    };
    // if d.contains(&(x_t, y_tot)) {
    //     return (true, x_t.abs() + y_tot.abs());
    // };
    (false, x_t.abs() + y_tot.abs())
}
fn walk_y(d: &mut HashSet<(i64, i64)>, y_tot: i64, y: i64, x_tot: i64) -> (bool, i64) {
    let mut y_t: i64 = y_tot;
    if y < 0 {
        for n in 1..=y.abs() {
            y_t += -1;
            if d.contains(&(x_tot, y_t)) {
                return (true, y_t.abs() + x_tot.abs());
            }
            d.insert((x_tot, y_t));
        }
    } else {
        for n in 1..=y.abs() {
            y_t += 1;
            if d.contains(&(x_tot, y_t)) {
                return (true, y_t.abs() + x_tot.abs());
            }
            d.insert((x_tot, y_t));
        }
    };
    // if d.contains(&(x_tot, y_t)) {
    //     return (true, y_t.abs() + x_tot.abs());
    // };
    (false, y_t.abs() + x_tot.abs())
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
                Self::Left => (Cardinal::West, -case, 0),
            },
            Cardinal::South => match self {
                Self::Right => (Cardinal::West, -case, 0),
                Self::Left => (Cardinal::East, case, 0),
            },
            Cardinal::West => match self {
                Self::Right => (Cardinal::North, 0, case),
                Self::Left => (Cardinal::South, 0, -case),
            },
            Cardinal::East => match self {
                Self::Right => (Cardinal::South, 0, -case),
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

    let ans = Instruction::first_double_visit(Cardinal::North, &d);
    println!("Part Two = {ans}")
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
    #[test]
    fn pt2_1() {
        let s = "R8, R4, R4, R8";
        let d = Instruction::from_str(s);
        let ans = Instruction::first_double_visit(Cardinal::North, &d);
        assert_eq!(ans, 4)
    }
}
