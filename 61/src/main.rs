use std::io::{self, Write};
fn num_to_b64(n:u8)-> char{
    match n {
        0..=25  => (b'A' + n) as char,//b'A'= 65 + 0 = 65 → 'A', b'A' is a byte literal
        26..=51 => (b'a' + (n - 26)) as char,//b'a' = 97 + (26 - 26) = 97 → 'a'
        52..=61 => (b'0' + (n - 52)) as char,//b'0' = 48 + (52 - 52) = 48 → '0'
        62      => '+',//62 => '+'
        63      => '/',//63 => '/'
        _       => panic!("Out of range"),//_ means “anything else.” panic! stops the program with an error message.
    }
}



fn main2() {
    println!("Character to write:");
    let mut val = std::string::String::new();
    std::io::stdin().read_line(&mut val).unwrap();

    let byte: u8 = val.trim().parse().unwrap();
    let buffer = [byte; 1];

    println!("Number of times to write:");
    let mut count = std::string::String::new();
    std::io::stdin().read_line(&mut count).unwrap();

    let count: usize = count.trim().parse().unwrap();

    let mut file = std::fs::File::create("out.txt").unwrap();

    for _i in 0..count {
        std::io::Write::write_all(&mut file, &buffer).unwrap();
    }
}
fn main(){

    let filename = std::env::args().nth(1).expect("Usage: cargo run -- <filename>");
    
    
    let bytes = std::fs::read(&filename).expect("Failed to read file");
    let mut output = String::new();

    let mut i = 0;
    while i < bytes.len() {
        let chunk = &bytes[i..std::cmp::min(i+3, bytes.len())];//3-byte groups (24 bits)

        let b0 = chunk.get(0).copied().unwrap_or(0);//.get() safely fetches a byte (returns Option<&u8>).,
                                                    //.copied() converts &u8 → u8., .unwrap_or(0) means “if missing, pretend it’s 0”.
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);

        let c0 = (b0 >> 2) & 0x3F;
        let c1 = ((b0 << 4) | (b1 >> 4)) & 0x3F;
        let c2 = ((b1 << 2) | (b2 >> 6)) & 0x3F;
        let c3 = b2 & 0x3F;//splits 3 bytes (24 bits) into 4 groups of 6 bits.

        output.push(num_to_b64(c0));
        output.push(num_to_b64(c1));

        if chunk.len() > 1 {
            output.push(num_to_b64(c2));
        } else {
            output.push('=');
        }

        if chunk.len() > 2 {
            output.push(num_to_b64(c3));
        } else {
            output.push('=');
        }

        i += 3;
    }//Always add first two Base64 chars. If chunk has only 1 byte, the last two are "==". If chunk has 2 bytes, only one "=".
     //= is the padding character for Base64. Then move to next chunk


    for line in output.as_bytes().chunks(76) {
        println!("{}", std::str::from_utf8(line).unwrap());
    }//Base64 traditionally wraps at 76 characters per line (per RFC 2045 / MIME spec). .chunks(76) splits the output bytes into lines. 
     //Convert back to UTF-8 text and print.
}
