use NumericalIO:: *;
// Helpers: Add/sub magnitudes (absolute values) of two numbers.
// "aug" and "add" are short for "augend" and "addend"
pub fn h_to_ix(val: &String) -> ix {
    let num = val.parse::<u64>().unwrap_or(0);
    return ix {
        sign: false,
        vals: vec![num],
    };
}
fn add_ix(a: &ix, b: &ix) -> ix{
       let mut carry = 0;
    let mut result: Vec<u64> = Vec::new();

    // Work from right to left
    let mut i = a.vals.len() as isize - 1;
    let mut j = b.vals.len() as isize - 1;

    while i >= 0 || j >= 0 || carry > 0 {
        let da = if i >= 0 { a.vals[i as usize] } else { 0 };
        let db = if j >= 0 { b.vals[j as usize] } else { 0 };
        let sum = da + db + carry;
        carry = sum / 10;
        result.push(sum % 10);
        i -= 1;
        j -= 1;
    }

    result.reverse(); // Reverse to restore correct order

    ix {
        sign: a.sign, // sign logic to be improved later
        vals: result,
    }


}
// "min" and "sub" are short for "minuend" and "subtrahend"
fn sub_ix(a: &ix, b: &ix) -> ix{
    let b = ix {
        sign: !b.sign,
        vals: b.vals.clone(),
    };
    return add_ix(&a, &b);}
// Compute the "greater than or equal" between two values.
fn mul_ix(a: &ix, b: &ix) -> ix{
ix {
        sign: a.sign ^ b.sign,
        vals: vec![0], // TODO: implement later
    }
}
fn main()-> ix {
     let args: Vec<String> = std::env::args().collect();
     let a: ix = h_to_ix(&args[1]);
     let b: ix = h_to_ix(&args[2]);
     match args[3].as_str() {
        "ADD" => &add_ix(&a, &b),
        "SUB" => &sub_ix(&a, &b),
        "MUL" => todo!(),
        "DIV" => todo!(),
        "REM" => todo!(),
    }
}
