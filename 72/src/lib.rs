// bignum/lib.rs
#[derive(Clone, Debug)]
pub struct ix {
    pub sign: i8,       // 1 for positive, -1 for negative
    pub vals: Vec<u8>,  // base-256 representation, least significant byte last
}

// Convert hex string → ix
pub fn h2i_ix(s: &str) -> ix {
    let mut vals = Vec::new();
    let mut bytes = s.trim().to_string();

    // Allow "0x" prefix
    if bytes.starts_with("0x") || bytes.starts_with("0X") {
        bytes = bytes[2..].to_string();
    }

    // If odd number of hex digits, pad
    if bytes.len() % 2 != 0 {
        bytes = format!("0{}", bytes);
    }

    for i in (0..bytes.len()).step_by(2) {
        let byte = u8::from_str_radix(&bytes[i..i+2], 16).unwrap_or(0);
        vals.push(byte);
    }

    ix { sign: 1, vals }
}

// Print ix as hex
pub fn see_ix(n: &ix) {
    if n.sign < 0 {
        print!("-");
    }

    for b in &n.vals {
        print!("{:02X}", b);
    }

    println!();
}

// Remove leading zeros
fn trim(mut v: Vec<u8>) -> Vec<u8> {
    while v.len() > 1 && v[0] == 0 {
        v.remove(0);
    }
    v
}

// ADD
pub fn add_ix(a: &ix, b: &ix) -> ix {
    // (Assumes positive numbers for now)
    let mut carry = 0u16;
    let mut result = Vec::new();

    let mut ai = a.vals.clone();
    let mut bi = b.vals.clone();

    // Make lengths equal
    while ai.len() < bi.len() {
        ai.insert(0, 0);
    }
    while bi.len() < ai.len() {
        bi.insert(0, 0);
    }

    for i in (0..ai.len()).rev() {
        let sum = ai[i] as u16 + bi[i] as u16 + carry;
        result.insert(0, (sum & 0xFF) as u8);
        carry = sum >> 8;
    }

    if carry > 0 {
        result.insert(0, carry as u8);
    }

    ix { sign: 1, vals: trim(result) }
}

// SUB (assuming a >= b)
pub fn sub_ix(a: &ix, b: &ix) -> ix {
    let mut result = Vec::new();
    let mut borrow = 0i16;

    let mut ai = a.vals.clone();
    let mut bi = b.vals.clone();
    let mut sign = 1;
    if lt_ix(&ix { sign: 1, vals: ai.clone() }, &ix { sign: 1, vals: bi.clone() }) {
        std::mem::swap(&mut ai, &mut bi);
        sign = -1;
    }
    // Equalize lengths
    while ai.len() < bi.len() {
        ai.insert(0, 0);
    }
    while bi.len() < ai.len() {
        bi.insert(0, 0);
    }
    let mut negative = false;
    if lt_ix(&ix { sign: 1, vals: ai.clone() }, &ix { sign: 1, vals: bi.clone() }) {
        std::mem::swap(&mut ai, &mut bi);
        negative = true;
    }

    for i in (0..ai.len()).rev() {
        let mut diff = ai[i] as i16 - bi[i] as i16 - borrow;
        if diff < 0 {
            diff += 256;
            borrow = 1;
        } else {
            borrow = 0;
        }
        result.insert(0, diff as u8);
    }

    ix { sign: if negative { -1 } else { 1 },
        vals: trim(result), }
}
fn lt_ix(a: &ix, b: &ix) -> bool {
    if a.vals.len() != b.vals.len() {
        return a.vals.len() < b.vals.len();
    }
    for (&x, &y) in a.vals.iter().zip(b.vals.iter()) {
        if x != y { return x < y; }
    }
    false
}

// TODO placeholders
pub fn mul_ix(a: &ix, b: &ix) -> ix {
    let len_a = a.vals.len();
    let len_b = b.vals.len();
    let mut result = vec![0u8; len_a + len_b]; // max possible length

    // Schoolbook multiply: start from least significant byte
    for i in (0..len_a).rev() {
        let mut carry = 0u16;
        for j in (0..len_b).rev() {
            let r_idx = result.len() - 1 - ((len_a - 1 - i) + (len_b - 1 - j));
            let prod = a.vals[i] as u16 * b.vals[j] as u16 + result[r_idx] as u16 + carry;
            result[r_idx] = (prod & 0xFF) as u8;
            carry = prod >> 8;
        }
        // carry overflow
        let r_idx = result.len() - 1 - ((len_a - 1 - i) + len_b);
        if carry > 0 {
            result[r_idx] += carry as u8;
        }
    }

    ix {
        sign: a.sign * b.sign,
        vals: trim(result),
    }
}

pub fn div_ix(_a: &ix, _b: &ix) -> ix {
    todo!("Implement division")
}

pub fn rem_ix(_a: &ix, _b: &ix) -> ix {
    todo!("Implement remainder")
}

