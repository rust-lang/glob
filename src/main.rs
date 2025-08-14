extern crate glob;
use glob::{glob_with, MatchOptions};

fn main() {
    let options = MatchOptions::new().glob_tilde_expansion(true);
    for v in glob_with("~eagle/*", options).unwrap() {
        println!("> {:?}", v.unwrap());
    }
}
