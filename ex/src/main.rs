fn print_red(s:String) {
    // Some terminal hacking nonsense for colors
    println!("\u{001b}[31m{s}\u{001b}[0m");
}

fn print_grn(s:String) {
    // More nonsense but 31 -> 32
    println!("\u{001b}[32m{s}\u{001b}[0m");
}

fn main() {
    let s = String::from("6");
    print_red(s.clone());
    print_grn(s.clone());
    println!("{s}")
}
