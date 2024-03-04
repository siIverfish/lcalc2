use std::fs;

use lcalc2::run;

fn main() {
    let path = "./lc/identities.lc";
    let input = fs::read_to_string(path).expect("able to read file");

    let output = run(&input).expect("i ∈ Λ");
    println!("Final result: {:#?}", output);
}
