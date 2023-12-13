use std::collections::HashMap;
use std::usize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Spring(Vec<Condition>);
impl Spring {
    fn as_str(&self) -> String {
        self.0.iter().map(Condition::as_char).collect::<String>()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        let spring = spring.chars().map(Condition::from_char).collect();
        let report = report
            .split(',')
            .map(|c| c.parse::<u8>().unwrap())
            .collect();
        HotSpring {
            spring: Spring(spring),
            report: Report(report),
            arrangements: None,
        }
    }
    fn calc(&mut self) -> &mut Self {
        let mut v: HashMap<Spring, bool> = HashMap::new();
        le_recursion(
            &mut v,
            &self.spring,
            &self.report,
            self.report.0.len() as u8,
        );

        fn le_recursion(
            memo: &mut HashMap<Spring, bool>,
            s: &Spring,
            r: &Report,
            n: u8,
        ) -> HashMap<Spring, bool> {
            let pr = format!("{} - {}", s.as_str(), r.as_str());
            println!("{pr}");
            todo!()
        }

        fn move_window() -> Option<Spring> {
            todo!()
        }

        self
    }
    fn ans(&self) -> usize {
        self.arrangements.unwrap()
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
}
