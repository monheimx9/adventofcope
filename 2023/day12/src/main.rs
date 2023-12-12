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
}

struct HotSpring {
    spring: Vec<Condition>,
    report: Vec<u8>,
}
impl HotSpring {
    fn from_str(s: &str) -> Self {
        let (spring, report) = s.split_once(' ').unwrap();
        let spring = spring.chars().map(Condition::from_char).collect();
        let report = report
            .split(',')
            .map(|c| c.parse::<u8>().unwrap())
            .collect();
        HotSpring { spring, report }
    }
}

fn main() {
    println!("Hello, world!");
}
