use std::fs::File;
use std::io::{BufRead, BufReader};

fn convert(pass: &str) -> i32 {
  i32::from_str_radix(
    &pass
      .replace('B', "1")
      .replace('F', "0")
      .replace('R', "1")
      .replace('L', "0"),
    2,
  )
  .unwrap()
}

fn main() -> Result<(), std::io::Error> {
  let seats: Vec<_> = BufReader::new(File::open("input/5")?)
    .lines()
    .map(|x| convert(&x.unwrap()))
    .collect();
  let min = seats.iter().min().unwrap();
  let max = seats.iter().max().unwrap();
  let sum: i32 = seats.iter().sum();

  println!("{} {}", max, (min + max) * (max - min + 1) / 2 - sum);
  Ok(())
}
