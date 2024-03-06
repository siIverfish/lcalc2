use std::fs;

use lcalc2::run;

static PATH: &str = "./lc/identities.lc";

fn main() {
    let input = fs::read_to_string(PATH).expect("able to read file");

    match run(&input) {
        Ok(result) => println!("Final result: {result}"),
        Err(error) => println!("An error occurred: {error}")
    }
}
