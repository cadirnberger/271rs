fn b85(n:u8)-> char{
    if n > 84 {
        panic!("Out of range: {}", n);
    }
    // ASCII 33 ('!') + n
    (b'!' + n) as char

}

fn main() {
    let filename = std::env::args().nth(1).expect("Usage: cargo run -- <filename>");
    let bytes = std::fs::read(&filename).expect("Failed to read file");
    let mut output = String::new();

    output.push_str("<~");

    let mut i = 0;
    let mut line_len = 2;
    while i < bytes.len() {
        let chunk = &bytes[i..usize::min(i + 4, bytes.len())];

        let mut value: u32 = 0;
        for &b in chunk {
            value = (value << 8) + b as u32;
        }

        let padding = 4 - chunk.len();
        value <<= 8 * padding;

        let mut digits = [0u8; 5];
        for j in (0..5).rev() {
            digits[j] = (value % 85) as u8;
            value /= 85;
        }

        for j in 0..(5 - padding) {
            output.push(b85(digits[j]));
            line_len += 1;
            if line_len == 80 {
                output.push('\n');
                line_len = 0;
            }
        }

        i += 4;
    }
    if line_len + 2 > 80 {
        output.push('\n');
    }

    output.push_str("~>");
    print!("{}", output);

}
