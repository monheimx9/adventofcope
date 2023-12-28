use std::usize;

#[derive(PartialEq, Eq)]
enum Stone {
    Rock,
    Ash,
}
impl Stone {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => unreachable!("Not a valid Stone"),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Mountain {
    cluster: Vec<Vec<Stone>>,
}
impl Mountain {
    fn from_str(s: &str) -> Self {
        let ss = s.lines().collect::<Vec<_>>();
        let cluster: Vec<Vec<Stone>> = ss
            .iter()
            .map(|s| s.chars().map(Stone::from_char).collect::<Vec<Stone>>())
            .collect();
        Mountain { cluster }
    }
    fn find_mirror(&self) -> usize {
        fn find_hori(m: &Mountain, p: usize) -> usize {
            if p >= m.cluster.len() - 2 {
                return 0;
            }
            let mut is_broken: bool = false;
            if let Some(i) = m
                .cluster
                .windows(2)
                .skip(p)
                .enumerate()
                .find(|(_, a)| a[0] == a[1])
                .map(|(i, _)| i)
            {
                if is_greater(i, m.cluster.len()) {
                    for n in 0..m.cluster.len() - i {
                        if m.cluster[i + 1 - n] != m.cluster[i + n] {
                            is_broken = true;
                            break;
                        }
                    }
                } else {
                    for n in 0..i {
                        if m.cluster[i - n] != m.cluster[i + n] {
                            is_broken = true;

                            break;
                        }
                    }
                }
                if !is_broken {
                    return (i + 1) * 100;
                }
            }
            0
        }

        fn find_verti(m: &Mountain, p: usize) -> usize {
            if p >= m.cluster[0].len() - 2 {
                return 0;
            }
            let mut is_broken = false;
            for n in 0..m.cluster[0].len() - 1 {
                let a = collect_column(&m.cluster, n);
                let b = collect_column(&m.cluster, n + 1);

                if a == b {
                    if is_greater(n, m.cluster[0].len()) {
                        for o in 0..m.cluster[0].len() - n {
                            let a = collect_column(&m.cluster, n + 1 - o);
                            let b = collect_column(&m.cluster, o + n);

                            if a != b {
                                is_broken = true;
                                break;
                            }
                        }
                    } else {
                        for o in 0..n {
                            let a = collect_column(&m.cluster, o);
                            let b = collect_column(&m.cluster, o + 1);

                            if a != b {
                                is_broken = true;
                                break;
                            }
                        }
                    }
                    if !is_broken {
                        return n + 1;
                    }
                }
            }
            0
        }

        let o = find_hori(self, 0) + find_verti(self, 0);
        println!("{o}");
        o
    }
}

fn is_greater(i: usize, l: usize) -> bool {
    i >= l / 2
}

fn collect_column<T>(grid: &[Vec<T>], column_index: usize) -> Option<Vec<&T>> {
    if let Some(row) = grid.get(0) {
        if column_index >= row.len() {
            return None;
        }
    } else {
        return None;
    }

    let column: Vec<&T> = grid
        .iter()
        .filter_map(|row| row.get(column_index))
        .collect();
    Some(column)
}

struct LavaIsland {
    mountains: Vec<Mountain>,
}
impl LavaIsland {
    fn from_str(s: &str) -> Self {
        let ss = s.split("\n\n").collect::<Vec<_>>();
        LavaIsland {
            mountains: ss.into_iter().map(Mountain::from_str).collect(),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let s = include_str!("../input.txt");
    let m = LavaIsland::from_str(s);
    let i: usize = m.mountains.iter().map(|c| c.find_mirror()).sum();
    println!("Part one mirrors = {i}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_1() {
        let s = include_str!("../test.txt");
        let m = LavaIsland::from_str(s);
        let i: usize = m.mountains.iter().map(|c| c.find_mirror()).sum();
        assert_eq!(i, 405)
    }
}
