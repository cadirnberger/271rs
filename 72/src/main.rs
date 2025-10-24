use bignum::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let a = h2i_ix(&args[1]);
    let b = h2i_ix(&args[2]);
    match args[3].as_str() {
        "ADD" => see_ix(&add_ix(&a, &b)),
        "SUB" => see_ix(&sub_ix(&a, &b)),
        "MUL" => see_ix(&mul_ix(&a, &b)),
        &_ => todo!(),
    }
}
