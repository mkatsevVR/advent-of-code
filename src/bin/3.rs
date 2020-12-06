use std::fs::File;
use std::io::{BufRead, BufReader};

fn count(input: &Vec<String>, right: usize, down: usize) -> i64 {
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;
    while i < input.len() {
        let line = input[i].as_bytes();
        if line[j] == b'#' {
            count += 1;
        }
        j += right;
        if j >= line.len() {
            j -= line.len();
        }
        i += down;
    }
    count
}

fn main() -> Result<(), std::io::Error> {
    let input: Vec<_> = BufReader::new(File::open("input/3")?)
        .lines()
        .collect::<Result<_, _>>()?;
    println!(
        "{} {}",
        count(&input, 3, 1),
        count(&input, 1, 1)
            * count(&input, 3, 1)
            * count(&input, 5, 1)
            * count(&input, 7, 1)
            * count(&input, 1, 2)
    );
    Ok(())
}
