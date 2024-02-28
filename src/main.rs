
use lcalc2::parse;

fn main() {
    let input: &str = "λλ(λ2(1 1))(λ2(1 1))";
    println!("{:#?}", parse(input));
}