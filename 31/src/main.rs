fn two_hex(n:char, m:char) -> u8 {
    let n = u8::from_str_radix(&n.to_string(), 16).unwrap();
    let m = u8::from_str_radix(&m.to_string(), 16).unwrap();
    return n * 16 + m;
}

fn pairs(mut cs:std::str::Chars, vals:&mut Vec<u8>) -> Option<char> {
    return match cs.next() {
        Some('\n') => None,
        Some(n) => match pairs(cs, &mut vals) {  // if there's digits here, look for more
            Some('\n') => None,
            Some(m) => {vals:&mut Vec<u8>
                println!("{:x}", two_hex(n,m));  // If last is found, print this then last
                None             // if the last is found, return it
            }
            None => Some(n),         // no more letters - we are last, return n
        },
        None => None,                // required by `rustc` in case we don't input anything
    }
}

fn chars_to_vec(mut cs:std::str::Chars) -> Vec<u8> {
    let mut vals = Vec::new();
    // from old main
    cs.next();
    cs.next();
    if let Some(n) = pairs(cs, &mut vals){
        vals.push(two_hex('0',n));
    }
    return vals;
}


fn main() {
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).unwrap();
    let mut cs = guess.chars();
    cs.next();
    cs.next();
    dbg!(chars_to_vec(guess.chars()))

}
