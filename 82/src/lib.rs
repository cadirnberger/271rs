// src/lib.rs
use num_bigint::{BigInt, Sign};
use num_traits::{One, Zero, ToPrimitive, Euclid};
use num_integer::Integer;
use sha2::{Digest, Sha512};

// ----------------- Utilities -----------------

/// H(m: bytes) -> sha512 bytes
fn h(m: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(m);
    hasher.finalize().to_vec()
}

/// bit(h: bytes, i: int) -> int
fn bit(h_val: &[u8], i: usize) -> u8 {
    let byte = i / 8;
    let off = i % 8;
    if byte >= h_val.len() { 0 } else { (h_val[byte] >> off) & 1 }
}

/// expmod(b:int,e:int,m:int) -> int
pub fn expmod(b_val: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    if m.is_one() { return BigInt::zero(); }
    let mut base = b_val.mod_floor(m);
    let mut exp = e.clone();
    let mut result = BigInt::one();
    let zero = BigInt::zero();
    while exp > zero {
        if (&exp & BigInt::one()) == BigInt::one() { result = (&result * &base).mod_floor(m); }
        exp >>= 1;
        base = (&base * &base).mod_floor(m);
    }
    result
}

/// modular inverse
pub fn inv(x: &BigInt, q: &BigInt) -> BigInt {
    let a = x.mod_floor(q);
    if a.is_zero() { return BigInt::zero(); }
    let mut lm = BigInt::one();
    let mut hm = BigInt::zero();
    let mut low = a.clone();
    let mut high = q.clone();
    while low > BigInt::one() {
        let r = &high / &low;
        let nm = &hm - &(&lm * &r);
        let new = &high - &(&low * &r);
        hm = lm;
        lm = nm;
        high = low;
        low = new;
    }
    lm.mod_floor(q)
}

/// recover x from y
pub fn xrecover(y: &BigInt, q: &BigInt, d: &BigInt, i_const: &BigInt) -> BigInt {
    let y2 = (y * y).mod_floor(q);
    let num = (&y2 - BigInt::one()).mod_floor(q);
    let den = (d * &y2 + BigInt::one()).mod_floor(q);
    let inv_den = inv(&den, q);
    let x2 = (num * inv_den).mod_floor(q);

    let mut x = expmod(&x2, &((q + BigInt::one()) / BigInt::from(4)), q);

    if (&x * &x - x2).mod_floor(q) != BigInt::zero() {
        x = (&x * i_const).mod_floor(q);
    }

    if x.clone() & BigInt::one() != BigInt::zero() {
        x = q - x;
    }

    x.mod_floor(q)
}

// ----------------- Core -----------------

fn edwards(p: &Vec<BigInt>, q_val: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    let x1 = &p[0]; let y1 = &p[1];
    let x2 = &q_val[0]; let y2 = &q_val[1];

    let x1y2 = (x1 * y2).mod_floor(q);
    let y1x2 = (y1 * x2).mod_floor(q);
    let x1x2 = (x1 * x2).mod_floor(q);
    let y1y2 = (y1 * y2).mod_floor(q);

    let dx1x2y1y2 = (d * &x1x2 * &y1y2).mod_floor(q);

    let num_x = (&x1y2 + &y1x2).mod_floor(q);
    let den_x = (&BigInt::one() + &dx1x2y1y2).mod_floor(q);
    let num_y = (&y1y2 - &x1x2).mod_floor(q);
    let den_y = (&BigInt::one() - &dx1x2y1y2).mod_floor(q);

    let x3 = (num_x * inv(&den_x, q)).mod_floor(q);
    let y3 = (num_y * inv(&den_y, q)).mod_floor(q);

    vec![x3, y3]
}

fn scalarmult(p: &Vec<BigInt>, e: &BigInt, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    let mut e = e.clone();
    let zero = BigInt::zero();
    let mut result = vec![BigInt::zero(), BigInt::one()];
    let mut addend = p.clone();
    while e > zero {
        if (&e & BigInt::one()) == BigInt::one() { result = edwards(&result, &addend, q, d); }
        addend = edwards(&addend, &addend, q, d);
        e >>= 1;
    }
    result
}

fn encodeint(y: &BigInt, b: usize) -> Vec<u8> {
    let bytes_len = b / 8;
    let (_sign, mut v) = y.to_bytes_le();
    v.resize(bytes_len, 0u8);
    v
}

fn encodepoint(p: &Vec<BigInt>, b: usize) -> Vec<u8> {
    let y_bytes = encodeint(&p[1], b);
    let x_lsb = (p[0].clone() & BigInt::one()).to_u8().unwrap_or(0);
    let mut out = y_bytes;
    let last_index = out.len() - 1;
    out[last_index] |= x_lsb << 7;
    out
}

pub fn publickey(sk: &[u8], b: usize, q: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {
    let digest = h(sk);
    let mut a_bytes = digest[..(b/8)].to_vec();
    a_bytes[0] &= 248;
    a_bytes[(b/8)-1] &= 127;
    a_bytes[(b/8)-1] |= 64;
    let a = BigInt::from_bytes_le(Sign::Plus, &a_bytes);
    let aB = scalarmult(b_point, &a, q, d);
    encodepoint(&aB, b)
}

fn hint(m: &[u8], _b: usize) -> BigInt {
    BigInt::from_bytes_le(Sign::Plus, &h(m))
}

pub fn signature(m: &[u8], sk: &[u8], pk: &[u8], b: usize, q: &BigInt, l: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {
    let h_sk = h(sk);
    let mut a_bytes = h_sk[..(b/8)].to_vec();
    a_bytes[0] &= 248;
    a_bytes[(b/8)-1] &= 127;
    a_bytes[(b/8)-1] |= 64;
    let a = BigInt::from_bytes_le(Sign::Plus, &a_bytes);
    let prefix = &h_sk[(b/8)..2*(b/8)];

    let mut r_in = Vec::new(); r_in.extend_from_slice(prefix); r_in.extend_from_slice(m);
    let r_big = hint(&r_in, b).mod_floor(l);

    let rB = scalarmult(b_point, &r_big, q, d);
    let r_enc = encodepoint(&rB, b);

    let mut hram_in = Vec::new(); hram_in.extend_from_slice(&r_enc); hram_in.extend_from_slice(pk); hram_in.extend_from_slice(m);
    let hram = hint(&hram_in, b).mod_floor(l);

    let s = (r_big + &hram * &a).mod_floor(l);
    let mut s_bytes = encodeint(&s, b);

    let mut sig = Vec::new();
    sig.extend_from_slice(&r_enc);
    sig.append(&mut s_bytes);
    sig
}

fn isoncurve(p: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> bool {
    let x = &p[0]; let y = &p[1];
    let x2 = (x.clone() * x.clone()).mod_floor(q);
    let y2 = (y.clone() * y.clone()).mod_floor(q);
    let lhs = (y2.clone() - x2.clone()).mod_floor(q);
    let rhs = (BigInt::one() + d * x2 * y2).mod_floor(q);
    lhs == rhs
}

fn decodeint(s: &[u8], b: usize) -> BigInt {
    let mut v = s.to_vec();
    if v.len() < (b/8) { v.resize(b/8, 0u8); }
    BigInt::from_bytes_le(Sign::Plus, &v[..(b/8)])
}

fn decodepoint(s: &[u8], b: usize, q: &BigInt, d: &BigInt, i_const: &BigInt) -> Result<Vec<BigInt>, &'static str> {
    if s.len() != b/8 { return Err("decodepoint length mismatch"); }
    let mut ys = s.to_vec();
    let last_index = ys.len() - 1;
    let sign = (ys[last_index] >> 7) & 1;
    ys[last_index] &= 0x7F;

    let y = decodeint(&ys, b).mod_floor(q);
    let x_cand = xrecover(&y, q, d, i_const);
    let final_x = if (x_cand.clone() & BigInt::one()).to_u8().unwrap_or(0) != sign { q - x_cand } else { x_cand };
    let p = vec![final_x.mod_floor(q), y];
    if !isoncurve(&p, q, d) { return Err("point not on curve"); }
    Ok(p)
}

pub fn checkvalid(s: &[u8], m: &[u8], pk: &[u8], b: usize, q: &BigInt, d: &BigInt, i_const: &BigInt, b_point: &Vec<BigInt>) -> bool {
    if s.len() != 2*(b/8) || pk.len() != b/8 { return false; }
    let r_enc = &s[..b/8]; let s_bytes = &s[b/8..];
    let a_pt = match decodepoint(pk, b, q, d, i_const) { Ok(p) => p, Err(_) => return false };
    let r_pt = match decodepoint(r_enc, b, q, d, i_const) { Ok(p) => p, Err(_) => return false };
    let s_int = decodeint(s_bytes, b);

    let l_suffix = BigInt::parse_bytes(b"27742317777372353535851937790883648493", 10).unwrap();
    let l_const = BigInt::from(2).pow(252) + l_suffix;
    if s_int >= l_const { return false; }

    let mut hram_in = Vec::new(); hram_in.extend_from_slice(r_enc); hram_in.extend_from_slice(pk); hram_in.extend_from_slice(m);
    let hram = hint(&hram_in, b).mod_floor(&l_const);

    let sB = scalarmult(b_point, &s_int, q, d);
    let hA = scalarmult(&a_pt, &hram, q, d);
    let rhs = edwards(&r_pt, &hA, q, d);

    sB[0].mod_floor(q) == rhs[0].mod_floor(q) && sB[1].mod_floor(q) == rhs[1].mod_floor(q)
}
