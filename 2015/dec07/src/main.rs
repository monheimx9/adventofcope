use std::collections::HashMap;

struct Wire {
    val: Option<u16>,
    op: String,
}

fn has_signal(w: &[Wire]) -> bool {
    w.iter().all(|v| v.val.is_some())
}

fn parse_instructions(s: &[&str]) -> HashMap<String, Wire> {
    let mut res: HashMap<String, Wire> = HashMap::with_capacity(s.len());
    for c in s {
        let (i, w) = c.split_once(" -> ").unwrap().to_owned();
        let val = i.parse::<u16>().ok();
        res.insert(
            w.to_string(),
            Wire {
                val,
                op: i.to_string(),
            },
        );
    }
    res
}

struct Ops {
    operator: Option<String>,
    wires: Option<Vec<String>>,
    signal: Option<u16>,
}

fn parse_ops(s: &str) -> Ops {
    todo!()
}

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    let i = parse_instructions(&input);
    println!("Hello, world!");
}
