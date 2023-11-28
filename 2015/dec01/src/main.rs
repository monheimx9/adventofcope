use std::{fs::File, io::Read};

fn main() {
    let mut i: i32 = 0;
    let mut f = File::open("input.txt").unwrap();
    let mut datastring = String::new();
    f.read_to_string(&mut datastring).unwrap();
    for c in datastring.chars() {
        match c {
            '(' => i += 1,
            ')' => i += -1,
            _ => i += 0,
        }
    }
    println!("The answer is {i}");
    i = 0;

    for (t, c) in datastring.chars().enumerate() {
        match c {
            '(' => i += 1,
            ')' => i += -1,
            _ => i += 0,
        };
        if i == -1 {
            println!("The basement is at position {}", t + 1);
            break;
        }
    }
}
