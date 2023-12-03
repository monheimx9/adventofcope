use grid::*;

enum Light {
    On,
    Off,
    Toggle,
}

struct Area {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

struct Instruction {
    a: Area,
    l: Light,
}

fn main() {
    println!("Hello, world!");
    let mut gr: Grid<bool> = Grid::new(1000, 1000);
    let txt: Vec<&str> = include_str!("../input.txt").lines().collect();
    let instruction = get_instructions(&txt);
    for i in instruction {
        push_button(&mut gr, &i.l, &i.a)
    }

    let count = gr.iter().filter(|&f| *f).count();

    println!("Total lights on {count}")
}

fn get_instructions(vec: &[&str]) -> Vec<Instruction> {
    let re = regex::Regex::new(r"(\d{1,3}),(\d{1,3}).+\s(\d{1,3}),(\d{1,3})$").unwrap();

    vec.iter()
        .map(|s| {
            let area = re.captures(s).unwrap();
            let area = Area {
                x1: area.get(1).unwrap().as_str().parse().unwrap(),
                y1: area.get(2).unwrap().as_str().parse().unwrap(),
                x2: area.get(3).unwrap().as_str().parse().unwrap(),
                y2: area.get(4).unwrap().as_str().parse().unwrap(),
            };
            Instruction {
                a: area,
                l: {
                    match s {
                        x if x.contains("turn on") => Light::On,
                        x if x.contains("turn off") => Light::Off,
                        x if x.contains("toggle") => Light::Toggle,
                        _ => Light::On,
                    }
                },
            }
        })
        .collect()
}

fn push_button(g: &mut Grid<bool>, l: &Light, a: &Area) {
    for n in a.x1..=a.x2 {
        for o in a.y1..=a.y2 {
            let m = g.get_mut(o as usize, n as usize).unwrap();
            match l {
                Light::On => *m = true,
                Light::Off => *m = false,
                Light::Toggle => *m ^= true,
            }
        }
    }
}
