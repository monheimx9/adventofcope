use std::cmp::min;
use std::fs::File;
use std::io::Read;
struct Dimension {
    pos1l: u32,
    pos2w: u32,
    pos3h: u32,
}
//2*l*w + 2*w*h + 2*h*l

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut datastring = String::new();
    f.read_to_string(&mut datastring).unwrap();
    let l: Vec<&str> = datastring.lines().collect();
    let dim: Vec<Dimension> = l
        .iter()
        .map(|a| {
            let u: Vec<&str> = a.split('x').collect();
            Dimension {
                pos1l: u[0].parse::<u32>().unwrap(),
                pos2w: u[1].parse::<u32>().unwrap(),
                pos3h: u[2].parse::<u32>().unwrap(),
            }
        })
        .collect();
    let mut areas: u32 = 0;
    let mut ribbon: u32 = 0;
    for d in dim {
        ribbon += d.pos1l * d.pos2w * d.pos3h;
        ribbon += smaller_sides(d.pos1l, d.pos2w, d.pos3h);
        let pos1l = 2 * d.pos1l * d.pos2w;
        let pos2w = 2 * d.pos2w * d.pos3h;
        let pos3h = 2 * d.pos3h * d.pos1l;
        let tot = pos1l + pos2w + pos3h + min(pos1l / 2, min(pos2w / 2, pos3h / 2));
        areas += tot;
    }
    println!("The total square feet is {}", areas);
    println!("The total ribbon meters needed are {}", ribbon);
}
fn smaller_sides(a: u32, b: u32, c: u32) -> u32 {
    let mut arr = [a, b, c];
    arr.sort();
    arr[0] + arr[0] + arr[1] + arr[1]
}
