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
        if let Some(i) = self
            .cluster
            .windows(2)
            .enumerate()
            .find(|(_, a)| a[0] == a[1])
            .map(|(i, _)| i)
        {
            if i > 0 && (self.cluster[i - 1] == self.cluster[i + 1]) {
                return (i + 1) * 100;
            }
        }

        for n in 0..self.cluster[0].len() - 1 {
            let a = &self.cluster[n][0..];
            let b = &self.cluster[n + 1][0..];

            if a == b {
                return n + 1;
            }
        }

        0
    }
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
