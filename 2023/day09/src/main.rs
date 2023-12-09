#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct History(Vec<Option<usize>>);
impl History {
    fn from_str(s: &str) -> Self {
        let mut first: Vec<Option<usize>> = s
            .split_whitespace()
            .map(|n| n.to_string().parse::<usize>().ok())
            .collect();
        if first.iter().all(|f| f.is_some()) {
            first.push(None);
        } else {
            panic!("Panic attack, not a valid list")
        }
        History(first)
    }
    fn find_last(&self) -> usize {
        let mut v: Vec<Vec<Option<usize>>> = Vec::new();
        for n in 0..self.0.len() {
            todo!()
        }
        todo!()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Report(Vec<History>);

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_1() {
        let i = "0 3 6 9 12 15";
        let h = History::from_str(i);
        println!("ok")
    }
}
