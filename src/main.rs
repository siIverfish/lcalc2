use std::{fs, path::Path};

use lcalc2::run;

static PATH: &str = "./lc/identities.lc";

fn main() {
    let path = Path::new(PATH);

    match run(path) {
        Ok(result) => println!("Final result: {result}"),
        Err(error) => println!("An error occurred: {error}"),
    }
}
