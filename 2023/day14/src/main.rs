use std::{default, time::Instant, usize};

use grid::*;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Rock {
    #[default]
    Empty,
    Round,
    Cube,
}
impl Rock {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Cube,
            'O' => Self::Round,
            _ => unreachable!("Not a valid rock"),
        }
    }
    fn as_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Cube => '#',
            Self::Round => 'O',
        }
    }
}

enum Cardinal {
    North,
    West,
    South,
    East,
}

struct Parabol(Grid<Rock>);
impl Parabol {
    fn from_str(s: &str) -> Self {
        let ss = s
            .lines()
            .map(|f| f.chars().map(Rock::from_char).collect::<Vec<Rock>>())
            .collect::<Vec<Vec<Rock>>>();

        let mut g: Grid<Rock> = Grid::new(ss.len(), ss[0].len());
        for r in 0..ss.len() {
            for c in 0..ss[0].len() {
                let cell = g.get_mut(r, c).unwrap();
                *cell = ss[r][c].clone();
            }
        }

        Parabol(g)
    }
    fn cycle(&mut self, n: usize) -> &mut Self {
        for o in 0..n {
            self.pull_rope(Cardinal::North);
            self.pull_rope(Cardinal::West);
            self.pull_rope(Cardinal::South);
            self.pull_rope(Cardinal::East);
            // print_grid(&self.0);
        }
        self
    }
    fn pull_rope(&mut self, car: Cardinal) -> &mut Self {
        let (rows, cols) = self.0.size();
        match car {
            Cardinal::North => {
                for c in 0..cols {
                    let mut empties: Vec<usize> = Vec::new();
                    for r in 0..rows {
                        match self.0[(r, c)] {
                            Rock::Round => {
                                if !empties.is_empty() {
                                    let case = self.0.get_mut(r, c).unwrap();
                                    *case = Rock::Empty;
                                    let case = self.0.get_mut(empties[0], c).unwrap();
                                    *case = Rock::Round;
                                    empties.remove(0);
                                    empties.push(r);
                                }
                            }
                            Rock::Cube => empties = Vec::new(),
                            Rock::Empty => empties.push(r),
                        }
                    }
                }
                // print_grid(&self.0);
            }
            Cardinal::South => {
                for c in 0..cols {
                    let mut empties: Vec<usize> = Vec::new();
                    for r in (0..rows).rev() {
                        match self.0[(r, c)] {
                            Rock::Round => {
                                if !empties.is_empty() {
                                    let case = self.0.get_mut(r, c).unwrap();
                                    *case = Rock::Empty;
                                    let case = self.0.get_mut(empties[0], c).unwrap();
                                    *case = Rock::Round;
                                    empties.remove(0);
                                    empties.push(r);
                                }
                            }
                            Rock::Cube => empties = Vec::new(),
                            Rock::Empty => empties.push(r),
                        }
                    }
                }
                // print_grid(&self.0);
            }
            Cardinal::East => {
                for r in 0..rows {
                    let mut empties: Vec<usize> = Vec::new();
                    for c in (0..cols).rev() {
                        match self.0[(r, c)] {
                            Rock::Round => {
                                if !empties.is_empty() {
                                    let case = self.0.get_mut(r, c).unwrap();
                                    *case = Rock::Empty;
                                    let case = self.0.get_mut(r, empties[0]).unwrap();
                                    *case = Rock::Round;
                                    empties.remove(0);
                                    empties.push(c);
                                }
                            }
                            Rock::Cube => empties = Vec::new(),
                            Rock::Empty => empties.push(c),
                        }
                    }
                }
                // print_grid(&self.0);
            }
            Cardinal::West => {
                for r in 0..rows {
                    let mut empties: Vec<usize> = Vec::new();
                    for c in 0..cols {
                        match self.0[(r, c)] {
                            Rock::Round => {
                                if !empties.is_empty() {
                                    let case = self.0.get_mut(r, c).unwrap();
                                    *case = Rock::Empty;
                                    let case = self.0.get_mut(r, empties[0]).unwrap();
                                    *case = Rock::Round;
                                    empties.remove(0);
                                    empties.push(c);
                                }
                            }
                            Rock::Cube => empties = Vec::new(),
                            Rock::Empty => empties.push(c),
                        }
                    }
                }
                // print_grid(&self.0);
            }
        }

        self
    }
    fn calc_weight(&self) -> usize {
        let mut count: usize = 0;
        for rows in 0..self.0.rows() {
            for cols in 0..self.0.cols() {
                if self.0[(rows, cols)] == Rock::Round {
                    count += self.0.rows() - rows;
                }
            }
        }
        count
    }
}

fn print_grid(g: &Grid<Rock>) {
    for n in 0..g.rows() {
        let line = g.iter_row(n).map(Rock::as_char).collect::<String>();
        println!("{}", line);
    }
    println!("--------------------------------------");
}

fn main() {
    println!("Hello, world!");
    let s = include_str!("../input.txt");
    let ins = Instant::now();
    let ans = Parabol::from_str(s)
        .pull_rope(Cardinal::North)
        .calc_weight();
    println!("Total weight part one is {}", ans);
    println!("Executed in {} micros", ins.elapsed().as_micros());
    let ins = Instant::now();
    let ans = Parabol::from_str(s).cycle(1000).calc_weight();
    println!("Total weight part two is {}", ans);
    println!("Executed in {} micros", ins.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1() {
        let s = include_str!("../test.txt");
        let ans = Parabol::from_str(s)
            .pull_rope(Cardinal::North)
            .calc_weight();
        assert_eq!(ans, 136)
    }
    #[test]
    fn pt2() {
        let s = include_str!("../test.txt");
        let ans = Parabol::from_str(s).cycle(1000).calc_weight();
        assert_eq!(ans, 64)
    }
}
