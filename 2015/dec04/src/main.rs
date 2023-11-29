fn main() {
    let k = "iwrupvqb";
    println!("Hello, world!");
    for i in 0..100000000 {
        let clef = format!("{k}{i}");
        let digest = md5::compute(&clef);
        let digest = format!("{:x}", digest);
        if digest.starts_with("000000") {
            println!("{digest}");
            println!("{clef}");
            break;
        }
    }
}
