// src/main.rs
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process::exit;

/// SHA-512 implementation (pure Rust).
///
/// Usage: cargo run --release -- <filename>
fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: sha512 <filename>");
        exit(2);
    });

    let data = match read_file_bytes(&filename) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error reading {}: {}", filename, e);
            exit(1);
        }
    };

    let digest = sha512(&data);
    // print as 128 hex digits lowercase, then two spaces, then filename (same format as sha512sum)
    print_hex(&digest);
    println!("  {}", filename);
}

fn read_file_bytes(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

/// Print bytes as lowercase hex with no separators.
fn print_hex(bytes: &[u8]) {
    for b in bytes {
        print!("{:02x}", b);
    }
}

/// Compute SHA-512 digest; returns 64 bytes.
fn sha512(message: &[u8]) -> [u8; 64] {
    // Initial hash values (first 64 bits of the fractional parts of the square roots of the first 8 primes)
    let mut h: [u64; 8] = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];

    // SHA-512 constants K (first 64 bits of the fractional parts of the cube roots of the first 80 primes)
    const K: [u64; 80] = [
        0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
        0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
        0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
        0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
        0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
        0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
        0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
        0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
        0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
        0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
        0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
        0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
        0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
        0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
    ];

    // Pre-processing: padding
    // Make a copy we can extend
    let mut padded = message.to_vec();
    // length in bits as u128 (SHA-512 uses 128-bit length)
    let bit_len = (message.len() as u128) * 8u128;
    // append 0x80
    padded.push(0x80);
    // append 0x00 bytes until message length (in bytes) mod 128 == 112
    while (padded.len() % 128) != 112 {
        padded.push(0x00);
    }
    // append 128-bit big-endian length
    let mut len_bytes = [0u8; 16];
    len_bytes[..16].copy_from_slice(&bit_len.to_be_bytes());
    padded.extend_from_slice(&len_bytes);

    // Process the message in successive 1024-bit chunks (128 bytes)
    assert!(padded.len() % 128 == 0);
    let chunks = padded.len() / 128;
    for i in 0..chunks {
        let chunk = &padded[i * 128..(i + 1) * 128];

        // Prepare the message schedule W
        let mut w = [0u64; 80];
        // first 16 words are directly from the chunk (big-endian)
        for t in 0..16 {
            let j = t * 8;
            w[t] = u64::from_be_bytes([
                chunk[j], chunk[j + 1], chunk[j + 2], chunk[j + 3],
                chunk[j + 4], chunk[j + 5], chunk[j + 6], chunk[j + 7],
            ]);
        }
        // remaining words
        for t in 16..80 {
            let s0 = small_sigma0(w[t - 15]);
            let s1 = small_sigma1(w[t - 2]);
            w[t] = w[t - 16]
                .wrapping_add(s0)
                .wrapping_add(w[t - 7])
                .wrapping_add(s1);
        }

        // Initialize working variables
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        for t in 0..80 {
            let t1 = hh
                .wrapping_add(big_sigma1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(K[t])
                .wrapping_add(w[t]);
            let t2 = big_sigma0(a).wrapping_add(maj(a, b, c));
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        // Add the compressed chunk to the current hash value
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    // Produce the final hash value (big-endian)
    let mut out = [0u8; 64];
    for (i, word) in h.iter().enumerate() {
        out[i * 8..(i + 1) * 8].copy_from_slice(&word.to_be_bytes());
    }
    out
}

/// Rotate right
#[inline(always)]
fn rotr(x: u64, n: u64) -> u64 {
    (x >> n) | (x << (64 - n))
}

/// Small sigma0
#[inline(always)]
fn small_sigma0(x: u64) -> u64 {
    rotr(x, 1) ^ rotr(x, 8) ^ (x >> 7)
}

/// Small sigma1
#[inline(always)]
fn small_sigma1(x: u64) -> u64 {
    rotr(x, 19) ^ rotr(x, 61) ^ (x >> 6)
}

/// Big Sigma0
#[inline(always)]
fn big_sigma0(x: u64) -> u64 {
    rotr(x, 28) ^ rotr(x, 34) ^ rotr(x, 39)
}

/// Big Sigma1
#[inline(always)]
fn big_sigma1(x: u64) -> u64 {
    rotr(x, 14) ^ rotr(x, 18) ^ rotr(x, 41)
}

/// Choice
#[inline(always)]
fn ch(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ ((!x) & z)
}

/// Majority
#[inline(always)]
fn maj(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}

