use std::collections::HashMap;
use std::time::Instant;
use std::usize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Condition {
    Functional,
    Damaged,
    Unknown,
}
impl Condition {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Functional,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!("Invalid Condition Char"),
        }
    }
    fn as_char(&self) -> char {
        match self {
            Self::Functional => '.',
            Self::Damaged => '#',
            Self::Unknown => '?',
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Spring(Vec<Condition>);
impl Spring {
    fn as_str(&self) -> String {
        self.0.iter().map(Condition::as_char).collect::<String>()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Report(Vec<u8>);
impl Report {
    fn as_str(&self) -> String {
        self.0
            .iter()
            .map(|f| format!("{},", f.to_string()))
            .collect::<String>()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HotSpring {
    spring: Spring,
    report: Report,
    arrangements: Option<usize>,
}
impl HotSpring {
    fn from_str(s: &str) -> Self {
        let (spring, report) = s.split_once(' ').unwrap();
        let mut spring: Vec<Condition> = spring.chars().map(Condition::from_char).collect();
        let report = report
            .split(',')
            .map(|c| c.parse::<u8>().unwrap())
            .collect();
        spring.push(Condition::Functional);
        HotSpring {
            spring: Spring(spring),
            report: Report(report),
            arrangements: None,
        }
    }
    fn expand(&mut self, n: u8) -> &mut Self {
        self.spring.0.pop();
        // self.spring.0.push(Condition::Unknown);
        let mut s = self.spring.0.clone();
        s.insert(0, Condition::Unknown);
        let r = self.report.0.clone();

        // println!("{ } - { }", self.spring.as_str(), self.report.as_str());

        self.spring
            .0
            .extend(s.iter().cloned().cycle().take(s.len() * n as usize));
        self.report
            .0
            .extend(r.iter().cloned().cycle().take(r.len() * n as usize));
        s.pop();

        self.spring.0.push(Condition::Functional);
        // println!("{ } - { }", self.spring.as_str(), self.report.as_str());

        self
    }
    fn calc(&mut self) -> &mut Self {
        let mut m: HashMap<(usize, usize), usize> = HashMap::new();
        let mut s = self.spring.clone();
        let mut r = self.report.clone();
        let pr = format!("{} - [{}]", s.as_str(), r.as_str());
        // println!("{pr}");
        self.arrangements = Some(le_recursion(0, 0, &mut m, &mut s, &mut r));

        fn le_recursion(
            p: usize,
            g: usize,
            memo: &mut HashMap<(usize, usize), usize>,
            s: &mut Spring,
            r: &mut Report,
        ) -> usize {
            let remaining_damaged = s.0.iter().skip(p).any(|f| matches!(f, Condition::Damaged));
            if g >= r.0.len() {
                if p < s.0.len() && remaining_damaged {
                    return 0;
                } else {
                    return 1;
                }
            }

            if p >= s.0.len() {
                return 0;
            }

            if let Some(x) = memo.get(&(p, g)) {
                return *x;
            }

            let mut res: usize = 0;
            let rs = r.0[g] as usize;

            if p + rs > s.0.len() {
                return 0;
            }

            match s.0[p] {
                Condition::Unknown => {
                    if !s.0[p..p + rs]
                        .iter()
                        .any(|f| matches!(f, Condition::Functional))
                        && !matches!(s.0[p + rs], Condition::Damaged)
                    {
                        res = le_recursion(p + rs + 1, g + 1, memo, s, r)
                            + le_recursion(p + 1, g, memo, s, r);
                    } else {
                        res = le_recursion(p + 1, g, memo, s, r);
                    }
                }
                Condition::Damaged => {
                    if !s.0[p..p + rs]
                        .iter()
                        .any(|f| matches!(f, Condition::Functional))
                        && !matches!(s.0[p + rs], Condition::Damaged)
                    {
                        res = le_recursion(p + rs + 1, g + 1, memo, s, r)
                    } else {
                        res = 0;
                    }
                }
                Condition::Functional => {
                    res = le_recursion(p + 1, g, memo, s, r);
                }
            }
            memo.insert((p, g), res);

            res
        }

        self
    }
    fn ans(&self) -> usize {
        self.arrangements.unwrap()
    }
}

fn main() {
    let i = Instant::now();
    println!("Hello, world!");
    let s = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let h = s
        .iter()
        .map(|f| HotSpring::from_str(f).calc().ans())
        .sum::<usize>();
    let o = i.elapsed().as_micros();
    println!("Part one completed in {o} micro-seconds");
    assert_eq!(h, 7221);
    let h = s
        .into_iter()
        .map(|f| HotSpring::from_str(f).expand(4).calc().ans())
        .sum::<usize>();
    assert_eq!(h, 7139671893722);
    let o = i.elapsed().as_micros();
    println!("Total duration: {o} micro-seconds")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_1() {
        let s = "???.### 1,1,3";
        let h = HotSpring::from_str(s).calc().ans();
        assert_eq!(h, 1)
    }

    #[test]
    fn pt1_2() {
        let s = ".??..??...?##. 1,1,3";
        let h = HotSpring::from_str(s).calc().ans();
        assert_eq!(h, 4)
    }
    #[test]
    fn pt1_3() {
        let s = "?#?#?#?#?#?#?#? 1,3,1,6";
        let h = HotSpring::from_str(s).calc().ans();
        assert_eq!(h, 1)
    }
    #[test]
    fn pt1_4() {
        let s = "????.#...#... 4,1,1";
        let h = HotSpring::from_str(s).calc().ans();
        assert_eq!(h, 1)
    }
    #[test]
    fn pt1_5() {
        let s = "????.######..#####. 1,6,5";
        let h = HotSpring::from_str(s).calc().ans();
        assert_eq!(h, 4)
    }
    #[test]
    fn pt1_6() {
        let s = "?###???????? 3,2,1";
        let h = HotSpring::from_str(s).calc().ans();
        assert_eq!(h, 10)
    }
    #[test]
    fn pt1_test() {
        let s = include_str!("../test.txt").lines().collect::<Vec<_>>();
        let h = s
            .into_iter()
            .map(|f| HotSpring::from_str(f).calc().ans())
            .sum::<usize>();
        assert_eq!(h, 21)
    }
    #[test]
    fn pt1() {
        let s = include_str!("../input.txt").lines().collect::<Vec<_>>();
        let h = s
            .into_iter()
            .map(|f| HotSpring::from_str(f).calc().ans())
            .sum::<usize>();
        assert_eq!(h, 7221)
    }
    #[test]
    fn pt2_1() {
        let s = include_str!("../test.txt").lines().collect::<Vec<_>>();
        let h = s
            .into_iter()
            .map(|g| HotSpring::from_str(g).expand(4).calc().ans())
            .sum::<usize>();
        assert_eq!(h, 525152)
    }
    #[test]
    fn pt2_2() {
        let s = "???.### 1,1,3";
        let h = HotSpring::from_str(s).expand(4).calc().ans();
        assert_eq!(h, 1)
    }
    #[test]
    fn pt2_3() {
        let s = "?###???????? 3,2,1";
        let h = HotSpring::from_str(s).expand(4).calc().ans();
        assert_eq!(h, 506250)
    }
    #[test]
    fn pt2() {
        let s = include_str!("../input.txt").lines().collect::<Vec<_>>();
        let h = s
            .into_iter()
            .map(|f| HotSpring::from_str(f).expand(4).calc().ans())
            .sum::<usize>();
        assert_eq!(h, 7139671893722)
    }
}
