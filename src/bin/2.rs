#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn check(line: &str) -> (i32, i32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
    let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let ltr = caps.get(3).unwrap().as_str().as_bytes()[0];
    let passwd = caps.get(4).unwrap().as_str().as_bytes();
    let count = passwd.iter().filter(|&&c| c == ltr).count();
    return (
        if count >= min && count <= max { 1 } else { 0 },
        if (passwd[min - 1] == ltr) ^ (passwd[max - 1] == ltr) {
            1
        } else {
            0
        },
    );
}

fn main() -> Result<(), std::io::Error> {
    let res = BufReader::new(File::open("input/2")?)
        .lines()
        .map(|x| check(&x.unwrap()))
        .fold((0, 0), |(a1, b1), (a2, b2)| ((a1 + a2), (b1 + b2)));
    println!("{:?}", res);
    Ok(())
}
