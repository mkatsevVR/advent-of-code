use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let values: Vec<_> = BufReader::new(File::open("input/1")?)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    'outer: for i in values.iter() {
        for j in values.iter() {
            if i + j == 2020 {
                println!("{}", i * j);
                break 'outer;
            }
        }
    }

    'outer2: for i in values.iter() {
        for j in values.iter() {
            for k in values.iter() {
                if i + j + k == 2020 {
                    println!("{}", i * j * k);
                    break 'outer2;
                }
            }
        }
    }
    Ok(())
}
