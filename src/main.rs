use std::path::Path;

use lcalc2::parse_file;
use lcalc2::run;
use lcalc2::preparse;
use lcalc2::parse;

static PATH: &str = "./lc/identities.lc";

fn main() {
    let path = Path::new(PATH);

    match run(path) {
        Ok(result) => println!("Final result: {result:?}"),
        Err(error) => println!("An error occurred: {error}"),
    }
}
