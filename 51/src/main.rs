use f16::f16;
fn i32_to_f16(n:i32) -> f16{
    let mut m = n as f32;
    let mut b::f16 = 0;
   let bits = m.to_bits();

    let sign = (bits >> 16) & 0x8000;
    let exp = ((bits >> 23) & 0xff) as i32;
    let frac = bits & 0x7fffff;

    if exp == 0 { // zero / subnormal
        return sign as u16;
    } else if exp == 0xff { // Inf/NaN
        if frac == 0 {
            return (sign | 0x7c00) as f16; // Inf
        } else {
            return (sign | 0x7e00) as f16; // NaN
        }
    }

    // normal case
    let new_exp = exp - 127 + 15;
    if new_exp >= 0x1f {
        // overflow -> Inf
        return (sign | 0x7c00) as f16;
    } else if new_exp <= 0 {
        // underflow -> 0
        return sign as f16;
    }

    let new_frac = (frac >> 13) & 0x3ff;
    (sign | ((new_exp as u32) << 10) | new_frac) as f16

}

fn println_f16(x:f16){
    println!("{}", i32_to_f16(x));
}
//fn add_f16s(x:f16, y:f16) -> f16{
//}
//fn sub_f16s(x:f16, y:f16) -> f16{
//}
//fn mul_f16s(x:f16, y:f16) -> f16{
//}
//fn div_f16s(x:f16, y:f16) -> f16{
//}


fn main() {
    
    println_f16(i32_to_f16(12));
}
