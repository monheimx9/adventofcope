#[derive(Debug, Copy, Clone)]
struct Seed {
    src: u64,
    dst: u64,
    r: u64,
}
impl Seed {
    fn from_str(s: &str) -> Vec<Seed> {
        let nums: Vec<u64> = s
            .split_whitespace()
            .filter_map(|f| f.parse::<u64>().ok())
            .collect();
        nums.iter()
            .map(|&f| Seed {
                src: f,
                dst: f,
                r: 0,
            })
            .collect()
    }
    fn from_str2(s: &str) -> Vec<Seed> {
        let nums: Vec<u64> = s
            .split_whitespace()
            .filter_map(|f| f.parse::<u64>().ok())
            .collect();
        let mut seeds: Vec<Seed> = Vec::new();
        let mut src: u64 = 0;
        let mut r: u64 = 0;
        for (i, n) in nums.iter().enumerate() {
            if i % 2 == 0 {
                src = *n;
            } else {
                r = *n;
                seeds.push(Seed { src, dst: src, r });
            }
        }
        seeds
    }
}

#[derive(Debug)]
struct Map {
    src: (u64, u64),
    dst: (u64, u64),
}

impl Map {
    fn from_str(s: &str) -> Self {
        let nums: Vec<u64> = s
            .split_whitespace()
            .filter_map(|f| f.parse::<u64>().ok())
            .collect();
        let (dst, src, r) = (nums[0], nums[1], nums[2]);
        let (dst, src) = ((dst, dst + r), (src, src + r));
        Map { src, dst }
    }
}

#[derive(Debug)]
struct Maps {
    items: Vec<Map>,
    name: String,
}

impl Maps {
    fn new(name: &str, item: &[&str]) -> Self {
        let items: Vec<Map> = item.iter().map(|f| Map::from_str(f)).collect();
        Maps {
            items,
            name: name.to_string(),
        }
    }
}

fn parse_input(inp: &[&str]) -> (Vec<Seed>, Vec<Maps>) {
    let seeds = Seed::from_str(inp[0]);
    let maps: Vec<Maps> = inp
        .iter()
        .skip(1)
        .map(|&s| {
            let ss: Vec<&str> = s.lines().collect();
            Maps::new(ss[0], &ss[1..])
        })
        .collect();
    (seeds, maps)
}
fn parse_input2(inp: &[&str]) -> (Vec<Seed>, Vec<Maps>) {
    let seeds = Seed::from_str2(inp[0]);
    let maps: Vec<Maps> = inp
        .iter()
        .skip(1)
        .map(|&s| {
            let ss: Vec<&str> = s.lines().collect();
            Maps::new(ss[0], &ss[1..])
        })
        .collect();
    (seeds, maps)
}

fn lowest_location(tutu: (&[Seed], &[Maps])) -> u64 {
    let mut iter_count: u64 = 0;
    let seeds = tutu.0;
    let maps = tutu.1;
    let mut r2: u64 = 999_999_999_999_999;
    for n in 0..seeds.len() {
        let mut seed = seeds[n];
        for o in 0..=seed.r {
            seed.dst = seed.src + o;
            for map in maps {
                if let Some(dst) = map.items.iter().find_map(|m| {
                    iter_count += 1;
                    in_range(m.src, m.dst, seed.dst)
                }) {
                    seed.dst = dst;
                }
            }
            if seed.dst < r2 {
                r2 = seed.dst;
                println!("Total iterations: {}", iter_count);
                println!(
                    "Lowest: {} | from Seed: {} (item: {}) | at range: {}",
                    r2, seed.src, n, o
                )
            }
        }
    }
    println!("Total iterations: {}", iter_count);
    r2
}
fn in_range(src: (u64, u64), dst: (u64, u64), seed: u64) -> Option<u64> {
    if seed >= src.0 && seed < src.1 {
        Some((seed - src.0) + dst.0)
    } else {
        None
    }
}

fn main() {
    let now = chrono::Utc::now();
    println!("{}", now.format("%b %-d, %-I:%M").to_string());
    println!("Hello, world!");
    let input: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();
    let (seeds, maps) = parse_input(&input);
    let loc = lowest_location((&seeds, &maps));
    println!("Lowest loc: {}", loc);

    let (seeds, maps) = parse_input2(&input);
    let loc = lowest_location((&seeds, &maps));
    println!("Lowed loc part 2: {}", loc);
    let now = chrono::Utc::now();
    println!("{}", now.format("%b %-d, %-I:%M").to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partone() {
        let input: Vec<&str> = include_str!("../test.txt").split("\n\n").collect();
        let (seeds, maps) = parse_input(&input);
        let loc = lowest_location((&seeds, &maps));
        println!("ok");
        assert_eq!(loc, 35)
    }
    #[test]
    fn parttwo() {
        let input: Vec<&str> = include_str!("../test.txt").split("\n\n").collect();
        let (seeds, maps) = parse_input2(&input);
        let loc = lowest_location((&seeds, &maps));
        println!("ok");
        assert_eq!(loc, 46)
    }
}
