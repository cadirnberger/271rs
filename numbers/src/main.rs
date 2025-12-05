fn main() {
    let args: Vec<String> = std::env::args().collect();
    let a:u8 = args[1].parse().unwrap();
    let b:u8 = args[2].parse().unwrap();
    dbg!(a + b);
}
