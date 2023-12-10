use core::panic;
use std::fs::File;
use std::io::Write;

use grid::*;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum PipeType {
    Verti,
    Hori,
    BendL,
    BendJ,
    Bend7,
    BendF,
    #[default]
    Groud,
    Start,
    Path(char),
    Nest,
    Junk,
}
impl PipeType {
    fn from_char(c: &char) -> Self {
        match c {
            '|' => Self::Verti,
            '-' => Self::Hori,
            'L' => Self::BendL,
            'J' => Self::BendJ,
            '7' => Self::Bend7,
            'F' => Self::BendF,
            '.' => Self::Groud,
            'S' => Self::Start,
            _ => panic!("Invalid pipe"),
        }
    }
    fn as_char(&self) -> char {
        match self {
            Self::Verti => '║',
            Self::Hori => '═',
            Self::BendL => '╚',
            Self::BendJ => '╝',
            Self::Bend7 => '╗',
            Self::BendF => '╔',
            Self::Groud => ' ',
            Self::Start => 'S',
            Self::Path(x) => *x,
            Self::Nest => '◉',
            Self::Junk => '▨',
        }
    }
    fn next_pipe(&self, d: &Direction) -> Direction {
        match self {
            Self::Verti => match d {
                Direction::North => Direction::North,
                Direction::South => Direction::South,
                _ => panic!(""),
            },
            Self::Hori => match d {
                Direction::West => Direction::West,
                Direction::East => Direction::East,
                _ => panic!(""),
            },
            Self::BendL => match d {
                Direction::South => Direction::East,
                Direction::West => Direction::North,
                _ => panic!(""),
            },
            Self::BendJ => match d {
                Direction::South => Direction::West,
                Direction::East => Direction::North,
                _ => panic!(""),
            },
            Self::Bend7 => match d {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                _ => panic!(""),
            },
            Self::BendF => match d {
                Direction::North => Direction::East,
                Direction::West => Direction::South,
                _ => panic!(""),
            },

            Self::Start => Direction::North,
            _ => panic!("Invalid next pipe"),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn next_xy(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::East => (1, 0),
        }
    }
}

struct BigMap(Grid<PipeType>);
impl BigMap {
    fn from_str(s: &str) -> Self {
        let li = s.lines().collect::<Vec<_>>();
        let mut g: Grid<PipeType> = Grid::new(li.len(), li[0].len());
        for (n, l) in li.iter().enumerate() {
            for (o, c) in l.chars().enumerate() {
                let curr = g.get_mut(n, o).unwrap();
                *curr = PipeType::from_char(&c);
            }
        }
        BigMap(g)
    }
    fn walk(&self) -> (isize, isize) {
        let mut g = self.0.clone();
        let (mut starty, mut startx) = (0, 0);
        for ((y, x), i) in g.indexed_iter() {
            if matches!(i, PipeType::Start) {
                starty = y;
                startx = x;
                break;
            }
        }
        let (mut y, mut x, mut d) = (starty, startx, Direction::North);
        let mut steps: usize = 0;

        loop {
            let pipe = g.get_mut(y, x).unwrap();
            if steps != 0 && matches!(pipe, PipeType::Start) {
                *pipe = PipeType::Path(pipe.as_char());
                break;
            }
            d = pipe.next_pipe(&d);
            let (nextx, nexty) = d.next_xy();
            y = (nexty + y as isize) as usize;
            x = (nextx + x as isize) as usize;
            if steps != 0 {
                *pipe = PipeType::Path(pipe.as_char());
            }
            steps += 1;
        }
        for n in 0..g.rows() {
            let mut inbound = false;
            let mut odd: usize = 0;
            for tile in g.iter_row_mut(n) {
                if matches!(tile, PipeType::Path(_)) {
                    let i = match tile {
                        PipeType::Path(x) => match x {
                            '║' => 1,
                            '╗' => 1,
                            '╔' => 1,
                            'S' => 0,
                            _ => 0,
                        },
                        _ => panic!("not a valid tile"),
                    };
                    odd += i;
                    inbound = odd % 2 != 0;
                    continue;
                }
                if inbound {
                    *tile = PipeType::Nest;
                } else {
                    *tile = PipeType::Junk;
                }
            }
        }
        print_grid(&g);

        (
            (steps as isize) / 2,
            g.iter().filter(|&p| matches!(p, PipeType::Nest)).count() as isize,
        )
    }
}

fn print_grid(g: &Grid<PipeType>) {
    let mut f = File::create("./result.txt").unwrap();
    for n in 0..g.rows() {
        let line = g.iter_row(n).map(PipeType::as_char).collect::<String>();
        writeln!(f, "{}", line).unwrap();
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let (s, n) = BigMap::from_str(s).walk();
    println!("Part one steps {s}");
    println!("Part two nests {n}");
    println!("Hello, world!");
}
