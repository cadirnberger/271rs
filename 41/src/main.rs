fn is_prime(val: i64) -> bool {
    if val <= 1 {
        return false;
    }
    let mut num = val - 1;
    while num > 1 {
        if val % num == 0 {
            return false;
        }
        num -= 1;
    }
    true
}

// Compute sqrt and print its 64-bit hex representation
fn print_sqrt_bits(val: i64) {
    let sqrt_val = f64::sqrt(val as f64);
    let frac = sqrt_val.fract();
    let bits = (frac * (1u64 << 64) as f64) as u64;
    let square = (bits as u128) * (bits as u128);
    println!("sqrt({:02}) = {:.8} -> {:016x} ^ 2 = {:018x}", val, sqrt_val, bits, square);
}

fn main() {
    for i in [2, 3, 5, 7, 11] {
        println!("{i}");
        print_sqrt_bits(i);
    }

    let mut cnt = 5;
    let mut val = 13;

    while cnt < 8 {
        if is_prime(val) {
            println!("{val}");
            print_sqrt_bits(val);
            cnt += 1;
        }
        val += 2; // skip even numbers
    }
}
