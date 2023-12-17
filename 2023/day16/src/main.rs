use std::collections::HashSet;
use std::thread;
use std::time::Instant;

const STACK_SIZE: usize = 24 * 1024 * 1024;

use grid::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Focuser {
    Empty(bool),
    RightMirror(bool),
    LeftMirror(bool),
    VertSplitter(bool),
    HoriSplitter(bool),
}
impl Default for Focuser {
    fn default() -> Self {
        Self::Empty(false)
    }
}
impl Focuser {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty(false),
            '/' => Self::RightMirror(false),
            '\\' => Self::LeftMirror(false),
            '|' => Self::VertSplitter(false),
            '-' => Self::HoriSplitter(false),
            _ => unreachable!("Not a valid Focuser"),
        }
    }
    fn as_char(&self) -> char {
        match self {
            Self::Empty(x) => {
                if *x {
                    '*'
                } else {
                    '.'
                }
            }
            Self::RightMirror(x) => {
                if *x {
                    '*'
                } else {
                    '/'
                }
            }
            Self::LeftMirror(x) => {
                if *x {
                    '*'
                } else {
                    '\\'
                }
            }
            Self::VertSplitter(x) => {
                if *x {
                    '*'
                } else {
                    '|'
                }
            }
            Self::HoriSplitter(x) => {
                if *x {
                    '*'
                } else {
                    '-'
                }
            }
        }
    }
    fn change_state(&self, state: bool) -> Self {
        match self {
            Self::Empty(_) => Self::Empty(state),
            Self::RightMirror(_) => Self::RightMirror(state),
            Self::LeftMirror(_) => Self::LeftMirror(state),
            Self::VertSplitter(_) => Self::VertSplitter(state),
            Self::HoriSplitter(_) => Self::HoriSplitter(state),
        }
    }
    fn get_dir(&self, d: &Direction) -> Vec<Direction> {
        match self {
            Self::Empty(_) => vec![*d],
            Self::RightMirror(_) => match d {
                Direction::North => vec![Direction::East],
                Direction::South => vec![Direction::West],
                Direction::West => vec![Direction::South],
                Direction::East => vec![Direction::North],
            },
            Self::LeftMirror(_) => match d {
                Direction::North => vec![Direction::West],
                Direction::South => vec![Direction::East],
                Direction::West => vec![Direction::North],
                Direction::East => vec![Direction::South],
            },
            Self::VertSplitter(_) => match d {
                Direction::North => vec![*d],
                Direction::South => vec![*d],
                _ => vec![Direction::North, Direction::South],
            },
            Self::HoriSplitter(_) => match d {
                Direction::West => vec![*d],
                Direction::East => vec![*d],
                _ => vec![Direction::East, Direction::West],
            },
        }
    }
    fn get_opposite(&self, d: &Direction) -> Direction {
        match self {
            Self::RightMirror(_) => match d {
                Direction::North => Direction::West,
                Direction::West => Direction::North,
                Direction::South => Direction::East,
                Direction::East => Direction::South,
            },
            Self::LeftMirror(_) => match d {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
            },
            Self::VertSplitter(_) => match d {
                Direction::South => Direction::North,
                Direction::North => Direction::South,
                Direction::West => Direction::East,
                Direction::East => Direction::West,
            },
            Self::HoriSplitter(_) => match d {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::West => Direction::East,
                Direction::East => Direction::West,
            },
            Self::Empty(_) => match d {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::West => Direction::East,
                Direction::East => Direction::West,
            },
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct BeamGrid(Grid<Focuser>);
impl BeamGrid {
    fn from_str(s: &str) -> Self {
        let ss = s
            .lines()
            .map(|f| f.chars().map(Focuser::from_char).collect::<Vec<Focuser>>())
            .collect::<Vec<Vec<Focuser>>>();

        let mut g: Grid<Focuser> = Grid::new(ss.len(), ss[0].len());
        for r in 0..ss.len() {
            for c in 0..ss[0].len() {
                let cell = g.get_mut(r, c).unwrap();
                *cell = ss[r][c];
            }
        }

        BeamGrid(g)
    }
    fn activate_beam(&mut self, r: usize, c: usize, d: Direction) -> &mut Self {
        fn nav(
            bg: &mut Grid<Focuser>,
            d: Direction,
            pos: (usize, usize),
            memo: &mut HashSet<(usize, usize, Direction)>,
        ) {
            let (r, c) = pos;
            if memo.contains(&(r, c, d)) {
                return;
            }
            memo.insert((r, c, d));
            // print_grid(bg);

            let (rows, cols) = bg.size();
            if r > rows - 1 || c > cols - 1 {
                return;
            }
            let mut new_d = d;
            let (mut new_r, mut new_c) = (r, c);

            loop {
                let cell = bg.get_mut(new_r, new_c).unwrap();
                *cell = cell.change_state(true);
                let cell = bg.get(new_r, new_c).unwrap().clone();
                let dirs = &cell.get_dir(&new_d);
                if dirs.len() > 1 {
                    for dir in dirs.iter() {
                        let opposite = &cell.get_opposite(dir);
                        if dir.can_move(new_r, new_c, rows, cols) {
                            let (next_r, next_c) = dir.pos(new_r, new_c);
                            memo.insert((next_r, next_c, *opposite));
                            nav(bg, *dir, (next_r, next_c), memo);
                        }
                    }
                    break;
                } else {
                    new_d = dirs[0];
                    // print_grid(bg);
                    if new_d.can_move(new_r, new_c, rows, cols) {
                        (new_r, new_c) = new_d.pos(new_r, new_c);
                    } else {
                        break;
                    }
                }
            }
        }
        let mut memo: HashSet<(usize, usize, Direction)> = HashSet::new();
        nav(&mut self.0, d, (r, c), &mut memo);
        print_grid(&self.0);
        self
    }
    fn calc_activated(&self) -> usize {
        self.0
            .iter()
            .filter(|&&f| match f {
                Focuser::Empty(x) => x,
                Focuser::LeftMirror(x) => x,
                Focuser::RightMirror(x) => x,
                Focuser::VertSplitter(x) => x,
                Focuser::HoriSplitter(x) => x,
            })
            .count()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}
impl Direction {
    fn pos(&self, r: usize, c: usize) -> (usize, usize) {
        match self {
            Self::North => (r.saturating_sub(1), c),
            Self::South => (r + 1, c),
            Self::West => (r, c.saturating_sub(1)),
            Self::East => (r, c + 1),
        }
    }
    fn can_move(&self, r: usize, c: usize, rt: usize, ct: usize) -> bool {
        match self {
            Self::North => r > 0,
            Self::South => r < rt - 1,
            Self::West => c > 0,
            Self::East => c < ct - 1,
        }
    }
}

fn print_grid(g: &Grid<Focuser>) {
    for n in 0..g.rows() {
        let line = g.iter_row(n).map(Focuser::as_char).collect::<String>();
        println!("{}", line);
    }
    println!("--------------------------------------");
}

fn run() {
    let s = include_str!("../input.txt");
    let ins = Instant::now();
    let ans = BeamGrid::from_str(s)
        .activate_beam(0, 0, Direction::East)
        .calc_activated();
    println!("Part one: {}", ans);
    println!("Executed in: {} micros", ins.elapsed().as_micros());
    let ins = Instant::now();
    let bg = BeamGrid::from_str(s);
    let (r, c) = bg.0.size();
    let mut ans: Vec<usize> = Vec::new();
    for n in 0..r {
        let mut bgc = bg.clone();
        ans.push(bgc.activate_beam(n, 0, Direction::East).calc_activated());
    }
    for n in 0..r {
        let mut bgc = bg.clone();
        ans.push(
            bgc.activate_beam(n, c - 1, Direction::West)
                .calc_activated(),
        );
    }
    for n in 0..c {
        let mut bgc = bg.clone();
        ans.push(bgc.activate_beam(0, n, Direction::South).calc_activated());
    }
    for n in 0..c {
        let mut bgc = bg.clone();
        ans.push(
            bgc.activate_beam(r - 1, n, Direction::North)
                .calc_activated(),
        );
    }
    println!("Part two: {}", ans.iter().max().unwrap());
    println!("Executed in: {} micros", ins.elapsed().as_micros());
}

fn main() {
    println!("Hello, world!");
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();
    child.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1() {
        let s = include_str!("../test.txt");
        let ans = BeamGrid::from_str(s)
            .activate_beam(0, 0, Direction::East)
            .calc_activated();
        assert_eq!(ans, 46)
    }
}
