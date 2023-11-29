use std::collections::HashMap;
use std::fs::{self};
fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut houses: HashMap<String, u32> = HashMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    houses.insert("0-0".to_string(), 0);
    for direction in file.chars() {
        match direction {
            '^' => y += 1,
            'v' => y += -1,
            '>' => x += 1,
            '<' => x += -1,
            _ => x += 0,
        }
        let k = format!("{x}-{y}");
        let count = houses.entry(k).or_insert(0);
        *count += 1;
    }
    let length = houses.len();

    println!("Total houses visited is {length}");

    //part 2
    let mut houses: HashMap<String, u32> = HashMap::new();
    houses.insert("0-0".to_string(), 0);
    let mut santa_x: i32 = 0;
    let mut santa_y: i32 = 0;
    let mut robo_x: i32 = 0;
    let mut robo_y: i32 = 0;

    for (i, direction) in file.chars().enumerate() {
        if i % 2 == 0 {
            match direction {
                '^' => robo_y += 1,
                'v' => robo_y += -1,
                '>' => robo_x += 1,
                '<' => robo_x += -1,
                _ => x += 0,
            }
        } else {
            match direction {
                '^' => santa_y += 1,
                'v' => santa_y += -1,
                '>' => santa_x += 1,
                '<' => santa_x += -1,
                _ => santa_y += 0,
            }
        }
        let k = format!("{robo_x}-{robo_y}");
        let count = houses.entry(k).or_insert(0);
        *count += 1;
        let k = format!("{santa_x}-{santa_y}");
        let count = houses.entry(k).or_insert(0);
        *count += 1;
    }
    let length = houses.len();
    println!("Total houses visited the second time is {length}")
}
