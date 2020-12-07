#[macro_use]
extern crate lazy_static;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;

fn check(cond: bool) -> Option<()> {
  return if cond { Some(()) } else { None };
}

fn validate(fields: &HashMap<String, String>) -> Option<()> {
  lazy_static! {
    static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref ECL: HashSet<&'static str> =
      "amb blu brn gry grn hzl oth".split_whitespace().collect();
    static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
  }

  let byr: i32 = fields.get("byr")?.parse().ok()?;
  check(byr >= 1920 && byr <= 2002)?;

  let iyr: i32 = fields.get("iyr")?.parse().ok()?;
  check(iyr >= 2010 && iyr <= 2020)?;

  let eyr: i32 = fields.get("eyr")?.parse().ok()?;
  check(eyr >= 2020 && eyr <= 2030)?;

  let hgt = fields.get("hgt")?;
  let hgt_val: i32 = hgt[..hgt.len() - 2].parse().ok()?;
  if hgt.ends_with("cm") {
    check(hgt_val >= 150 && hgt_val <= 193)?;
  } else if hgt.ends_with("in") {
    check(hgt_val >= 59 && hgt_val <= 76)?;
  } else {
    return None;
  }

  check(HCL.is_match(fields.get("hcl")?))?;

  check(ECL.contains(fields.get("ecl")?.as_str()))?;

  check(PID.is_match(fields.get("pid")?))?;

  Some(())
}

fn main() -> Result<(), std::io::Error> {
  let mut valid1 = 0;
  let mut valid2 = 0;
  let mut fields = HashMap::new();
  for line in BufReader::new(File::open("input/4")?).lines() {
    let line = line.unwrap();
    if line.is_empty() {
      fields.remove("cid");
      if fields.len() == 7 {
        valid1 += 1;
        if validate(&fields).is_some() {
          valid2 += 1;
        }
      }
      fields = HashMap::new();
    } else {
      for item in line.split_whitespace() {
        let (field, value) = item.split(':').next_tuple().unwrap();
        fields.insert(field.to_owned(), value.to_owned());
      }
    }
  }
  println!("{} {}", valid1, valid2);
  Ok(())
}
