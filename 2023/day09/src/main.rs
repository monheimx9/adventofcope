use std::result;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct History(Vec<Option<i64>>);
impl History {
    fn from_str(s: &str) -> Self {
        let mut first: Vec<Option<i64>> = s
            .split_whitespace()
            .map(|n| n.to_string().parse::<i64>().ok())
            .collect();
        if !first.iter().all(|f| f.is_some()) {
            panic!("Panic attack, not a valid list")
        }
        History(first)
    }
    fn find_last(&self) -> i64 {
        let mut v: Vec<Vec<Option<i64>>> = Vec::new();
        let mut index: usize = 0;
        let first = self.0[self.0.len() - 1].unwrap();
        v.push(self.0.clone());
        let mut new: Vec<Option<i64>> = self.0.clone();

        loop {
            if new.iter().all(|f| f.unwrap() == 0) {
                break;
            } else {
                new = History::get_diff(&v[index]);
                v.push(new.clone());
                index += 1;
            }
        }
        let mut remainder: Vec<i64> = Vec::new();
        for item in v.iter().skip(1) {
            remainder.push(item[item.len() - 1].unwrap())
        }
        let last: i64 = remainder.iter().sum();
        if first == 0 {
            first - last
        } else {
            first + last
        }
    }
    fn find_first(&self) -> i64 {
        let mut v: Vec<Vec<Option<i64>>> = Vec::new();
        let mut index: usize = 0;
        let first = self.0[self.0.len() - 1].unwrap();
        v.push(self.0.clone());
        let mut new: Vec<Option<i64>> = self.0.clone();

        loop {
            if new.iter().all(|f| f.unwrap() == 0) {
                break;
            } else {
                new = History::get_diff(&v[index]);
                v.push(new.clone());
                index += 1;
            }
        }
        let mut remainder: Vec<i64> = Vec::new();
        for item in v.iter().skip(1) {
            remainder.push(item[0].unwrap())
        }
        let last: i64 = remainder.iter().sum();
        first + last
    }
    fn get_diff(vals: &[Option<i64>]) -> Vec<Option<i64>> {
        vals.windows(2)
            .map(|a| Some(a[1].unwrap() - a[0].unwrap()))
            .collect()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Report(Vec<History>);
impl Report {
    fn from_str(s: &str) -> i64 {
        let ss: Vec<&str> = s.lines().collect();
        let res: Vec<i64> = ss
            .iter()
            .map(|f| History::from_str(f).find_last())
            .collect();
        res.iter().sum()
    }
}

fn main() {
    println!("Hello, world!");
    let i = include_str!("../input.txt");
    let h = Report::from_str(i);
    println!("Part one = {h}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_1() {
        let i = "0 3 6 9 12 15";
        let h = History::from_str(i).find_last();
        println!("ok");
        assert_eq!(h, 18)
    }
    #[test]
    fn pt1_2() {
        let i = "10 13 16 21 30 45";
        let h = History::from_str(i).find_last();
        assert_eq!(h, 68)
    }
    #[test]
    fn pt1_3() {
        let i = "-1 -2 -3 -4 -5 -6 -7 -8 -9 -10 -11 -12 -13 -14 -15 -16 -17 -18 -19 -20 -21";
        let h = History::from_str(i).find_last();
        assert_eq!(h, -22)
    }
    #[test]
    fn pt1_4() {
        let i = "3 -2 -7 -12 -17 -22 -27 -32 -37 -42 -47 -52 -57 -62 -67 -72 -77 -82 -87 -92 -97";
        let h = History::from_str(i).find_last();
        assert_eq!(h, -102)
    }
    #[test]
    fn pt1_5() {
        let i = "22 34 44 52 58 62 64 64 62 58 52 44 34 22 8 -8 -26 -46 -68 -92 -118";
        let h = History::from_str(i).find_last();
        assert_eq!(h, -146)
    }
    #[test]
    fn pt1_6() {
        let i = "12 9 6 3 0 -3 -6 -9 -12 -15 -18 -21 -24 -27 -30 -33 -36 -39 -42 -45 -48";
        let h = History::from_str(i).find_last();
        assert_eq!(h, -51)
    }
    #[test]
    fn pt1_7() {
        let i = "22 34 44 52 58 62 64 64 62 58 52 44 34 22 8 -8 -26 -46 -68 -92 -118
12 9 6 3 0 -3 -6 -9 -12 -15 -18 -21 -24 -27 -30 -33 -36 -39 -42 -45 -48";
        let h = Report::from_str(i);
        assert_eq!(h, (-197))
    }
    #[test]
    fn pt1_8() {
        let i = "14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 -1 -2 -3 -4 -5 -6";
        let h = History::from_str(i).find_last();
        assert_eq!(h, -7)
    }
    #[test]
    fn pt1_9() {
        let i = "-1 -1 -1 -1";
        let h = History::from_str(i).find_last();
        assert_eq!(h, -1)
    }
    #[test]
    fn pt1_10() {
        let i = "12 30 66 118 187 291 486 901 1807 3760 7885 16402 33536 67001 130303 246169 451478 804146 1392500 2347766 3860393";
        let h = History::from_str(i).find_last();
        assert_eq!(h, 6201039)
    }

    #[test]
    fn pt1_11() {
        let i = include_str!("../test.txt");
        let h = Report::from_str(i);
        assert_eq!(h, 114)
    }
    #[test]
    fn pt2_1() {
        let i = "10 13 16 21 30 45";
        let h = History::from_str(i).find_last();
        assert_eq!(h, 5)
    }
}
