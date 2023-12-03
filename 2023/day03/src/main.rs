use regex::{self, Regex};
struct Gears {
    x: i32,
    y: i32,
}

impl Gears {
    fn from_vecstr(vec: &[&str]) -> Vec<Gears> {
        let mut gears: Vec<Gears> = Vec::new();
        for (yl, l) in vec.iter().enumerate() {
            for (xc, c) in l.chars().enumerate() {
                if c == '*' {
                    gears.push(Gears {
                        x: xc as i32,
                        y: yl as i32,
                    })
                }
            }
        }
        gears
    }
}
struct PartNumber<'a> {
    x: i32,
    y: i32,
    lenght: u8,
    value: &'a str,
}

impl<'a> PartNumber<'a> {
    fn from_vecstr(vec: &[&'a str]) -> Vec<PartNumber<'a>> {
        let mut new_vec: Vec<PartNumber<'a>> = Vec::new();
        for (i, s) in vec.iter().enumerate() {
            let nums = extract_num_from_str(s);
            for num in nums {
                new_vec.push(PartNumber {
                    x: num.1,
                    y: i as i32,
                    lenght: (num.0.len() as u8),
                    value: num.0,
                });
            }
        }
        new_vec
    }
    fn int(&self) -> u32 {
        self.value.parse().unwrap()
    }
}

struct ValidPart<'b> {
    part: PartNumber<'b>,
}

impl<'b> ValidPart<'b> {
    fn from_entire_schema(p: &PartNumber<'b>, sch: &[&str]) -> Option<ValidPart<'b>> {
        let skip_index = {
            if p.y > 0 {
                p.y - 1
            } else {
                0
            }
        };
        for (il, l) in sch.iter().enumerate().skip(skip_index as usize) {
            for (ic, c) in l.chars().enumerate() {
                if !c.is_ascii_digit() && c != '.' {
                    if il as i32 >= (p.y - 1) && (il as i32 <= p.y + 1) {
                        if ic as i32 >= (p.x - 1) && ((ic as i32) <= (p.x + (p.lenght as i32))) {
                            return Some(ValidPart {
                                part: PartNumber {
                                    x: p.x,
                                    y: p.y,
                                    lenght: p.lenght,
                                    value: p.value,
                                },
                            });
                        }
                    }
                }
                if il as i32 > (p.y + 1) {
                    return None;
                }
            }
        }
        None
    }
}

struct Schema<'c>(Vec<ValidPart<'c>>, Vec<Gears>);

impl<'c> Schema<'c> {
    fn from_vecstr(vec: &[&'c str]) -> Schema<'c> {
        let gears: Vec<Gears> = Gears::from_vecstr(vec);
        let all_parts = PartNumber::from_vecstr(vec);
        Schema(
            all_parts
                .iter()
                .map(|f| ValidPart::from_entire_schema(f, vec))
                .filter(|f| f.is_some())
                .flatten()
                .collect(),
            gears,
        )
    }
    fn ratios(&self) -> u32 {
        let gears = &self.1;
        let parts = &self.0;
        let mut count: u32 = 0;

        'outer: for g in gears {
            let mut valid: Vec<u32> = Vec::new();
            for p in parts {
                let py = p.part.y;
                let px = p.part.x;
                let pl = p.part.lenght as i32;
                let pv = p.part.int();
                if py >= (g.y - 1) && py <= (g.y + 1) {
                    if px >= (g.x - pl) && px <= (g.x + 1) {
                        valid.push(pv);
                        if valid.len() == 2 {
                            count += valid[0] * valid[1];
                            continue 'outer;
                        }
                    }
                }
            }
        }
        count
    }
}

fn main() {
    println!("Hello, world!");
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    let p = Schema::from_vecstr(&input);
    let total: u32 = p.0.iter().map(|f| f.part.int()).sum();
    println!("The sum for part one is {total}");

    let total2 = p.ratios();

    println!("The sum for part two is {total2}");
}

fn extract_num_from_str(s: &str) -> Vec<(&str, i32)> {
    let re = Regex::new(r"([0-9]{1,3})").unwrap();
    re.captures_iter(s)
        .filter_map(|cap| {
            let num = cap.get(1)?.as_str();
            let start = cap.get(0)?.start() as i32;
            Some((num, start))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn partone() {
        let inp: Vec<&str> = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let p = Schema::from_vecstr(&inp);
        let sumy: u32 = p.0.iter().map(|f| f.part.int()).sum();
        assert_eq!(sumy, 4361)
    }
    #[test]
    fn partone2() {
        let input: Vec<&str> = vec!
            ["..172..............................454..46.......507..........809......923.778..................793..............137.............238........",
            "............*.........712........=.......*................515.*...........*.......690.........../..........658.........=.........*..........",
            ".........823.835........%.........710.....749........134..%............................#812...&.....925.../..........276.......386..........",
            "519..................13......341.................481....=.....$............-.......211.......92.......*.....................................",
            "............832*105..@........$..................*.........797.....535..932.........*....152...........123.........678.540...........-...6..",
            "....&..948..........................271....-....228..79.26.........................733...=...715............27.586........*.......883...*...",
            "..172.......=..+.............88&....%....340.55.....+.............465..398......=..................585.......*....*812...347................",];
        let p = Schema::from_vecstr(&input);
        let sumy: u32 = p.0.iter().map(|f| f.part.int()).sum();
        assert_eq!(sumy, 20313)
    }
    #[test]
    fn parttwo1() {
        let inp: Vec<&str> = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let p = Schema::from_vecstr(&inp);
        let sumy: u32 = p.ratios();
        assert_eq!(sumy, 467835)
    }
    #[test]
    fn parttwo2() {
        let input: Vec<&str> = vec!
            ["..172..............................454..46.......507..........809......923.778..................793..............137.............238........",
            "............*.........712........=.......*................515.*...........*.......690.........../..........658.........=.........*..........",
            ".........823.835........%.........710.....749........134..%............................#812...&.....925.../..........276.......386..........",
            "519..................13......341.................481....=.....$............-.......211.......92.......*.....................................",
            "............832*105..@........$..................*.........797.....535..932.........*....152...........123.........678.540...........-...6..",
            "....&..948..........................271....-....228..79.26.........................733...=...715............27.586........*.......883...*...",
            "..172.......=..+.............88&....%....340.55.....+.............465..398......=..................585.......*....*812...347................",];

        let p = Schema::from_vecstr(&input);
        let sumy: u32 = p.ratios();
        assert_eq!(sumy, 2660299)
    }
}
