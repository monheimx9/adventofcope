use grid::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

type GalaxyIndex = (u16, usize, usize);
type Pair = HashMap<u16, u32>;

#[derive(Default, Clone)]
enum Space {
    Galaxy(u16, Option<Pair>),
    #[default]
    Void,
}
impl Space {
    fn from_char(c: &char, id: u16) -> Self {
        match c {
            '.' => Self::Void,
            '#' => Self::Galaxy(id, None),
            _ => unreachable!("Invalid Space tile"),
        }
    }
    fn as_char(&self) -> char {
        match self {
            Space::Void => '.',
            Self::Galaxy(_, _) => '#',
        }
    }
    fn get_index(&self) -> u16 {
        match self {
            Self::Galaxy(i, _) => *i,
            _ => unreachable!("Can't get void index"),
        }
    }
}

struct Universe(Grid<Space>);
impl Universe {
    fn from_str(s: &str) -> Self {
        let mut count: u16 = 1;
        let array = s.lines().collect::<Vec<_>>();
        let g: Vec<Vec<Space>> = array
            .iter()
            .map(|line| {
                line.chars()
                    .map(|f| {
                        let sp = Space::from_char(&f, count);
                        if matches!(sp, Space::Galaxy(_, _)) {
                            count += 1;
                        }
                        sp
                    })
                    .collect::<Vec<Space>>() // Collect the inner iterator into Vec<Space>
            })
            .collect();
        let mut gr: Grid<Space> = Grid::new(g.len(), g[0].len());
        for n in 0..g.len() {
            for o in 0..g[0].len() {
                let space = gr.get_mut(o, n).unwrap();
                *space = g[o][n].clone();
            }
        }
        Universe(gr)
    }
    fn expand(&mut self, expansion: Option<usize>) -> &mut Self {
        for n in (0..self.0.rows()).rev() {
            if self.0.iter_row(n).all(|s| matches!(s, Space::Void)) {
                if let Some(e) = expansion {
                    (0..e).for_each(|_| self.0.insert_row(n, vec![Space::Void; self.0.cols()]));
                }
            }
        }
        for n in (0..self.0.cols()).rev() {
            if self.0.iter_col(n).all(|s| matches!(s, Space::Void)) {
                if let Some(e) = expansion {
                    (0..e).for_each(|_| self.0.insert_col(n, vec![Space::Void; self.0.rows()]));
                }
            }
        }
        //print_grid(&self.0);
        println!("Test");
        self
    }
    fn find_nearest(&self) -> usize {
        let gal_index: Vec<GalaxyIndex> = self
            .0
            .indexed_iter()
            .filter(|((_, _), g)| matches!(g, Space::Galaxy(_, _)))
            .map(|((y, x), s)| (s.get_index(), y, x))
            .collect();
        let mut steps: usize = 0;

        for n in 0..gal_index.len() {
            let (_, y, x) = gal_index[n];
            for galaxy in gal_index.iter().skip(n + 1) {
                let (_, y2, x2) = galaxy;
                steps += (*y2 as isize - y as isize).unsigned_abs()
                    + (*x2 as isize - x as isize).unsigned_abs();
            }
        }
        steps
    }
}
fn print_grid(g: &Grid<Space>) {
    let mut f = File::create("./result.txt").unwrap();
    for n in 0..g.rows() {
        let line = g.iter_row(n).map(Space::as_char).collect::<String>();
        writeln!(f, "{}", line).unwrap();
    }
}

fn main() {
    println!("Hello, world!");
    let s = include_str!("../input.txt");
    let u = Universe::from_str(s).expand(Some(1)).find_nearest();
    println!("Part one {u}");
    let mut u = Universe::from_str(s);
    let a = u.expand(None).find_nearest();
    let b = u.expand(Some(9)).find_nearest();
    let ans = not_fib_but_you_get_it(5, b, b - a);
    println!("Part two {ans}")
}

fn not_fib_but_you_get_it(n: u8, a: usize, r: usize) -> usize {
    if n == 0 {
        return a;
    };
    not_fib_but_you_get_it(n - 1, a + r * 10, r * 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_1() {
        let s = include_str!("../test.txt");
        let u = Universe::from_str(s).expand(Some(1)).find_nearest();
        assert_eq!(u, 374);
    }
    #[test]
    fn pt2_1() {
        let s = include_str!("../test.txt");
        let mut u = Universe::from_str(s);
        let a = u.expand(None).find_nearest();
        let b = u.expand(Some(9)).find_nearest();
        let ans = not_fib_but_you_get_it(1, b, b - a);
        assert_eq!(ans, 8410)
    }
    #[test]
    fn pt1() {
        let s = include_str!("../input.txt");
        let u = Universe::from_str(s).expand(Some(1)).find_nearest();
        assert_eq!(u, 9274989)
    }
    #[test]
    fn pt2() {
        let s = include_str!("../input.txt");
        let mut u = Universe::from_str(s);
        let a = u.expand(None).find_nearest();
        let b = u.expand(Some(9)).find_nearest();
        let ans = not_fib_but_you_get_it(5, b, b - a);
        assert_eq!(ans, 357134560737)
    }
}
