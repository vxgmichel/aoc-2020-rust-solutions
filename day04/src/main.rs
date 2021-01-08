use std::collections::HashMap;
use std::io::{self, BufRead};

fn split_field(x: &str) -> Option<(&str, &str)> {
    let v = x.split(':').collect::<Vec<_>>();
    match &v[..] {
        [a, b] => Some((a, b)),
        _ => None,
    }
}

fn check_1(s: String) -> Option<()> {
    // Create a hash map
    let fields = s
        .split(' ')
        .filter_map(|x| split_field(x))
        .collect::<HashMap<_, _>>();
    for field in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        fields.get(field)?;
    }
    Some(())
}

fn check_2(s: String) -> Option<()> {
    // Create a hash map
    let fields = s
        .split(' ')
        .filter_map(|x| split_field(x))
        .collect::<HashMap<_, _>>();
    // Check byr
    let byr: usize = fields.get("byr")?.parse().ok()?;
    if !(1920 <= byr && byr <= 2002) {
        return None;
    }
    // Check iyr
    let iyr: usize = fields.get("iyr")?.parse().ok()?;
    if !(2010 <= iyr && iyr <= 2020) {
        return None;
    }
    // Check eyr
    let eyr: usize = fields.get("eyr")?.parse().ok()?;
    if !(2020 <= eyr && eyr <= 2030) {
        return None;
    }
    // Check hgt
    let hgt = fields.get("hgt")?;
    let hgt_value: usize = hgt.get(..hgt.len() - 2)?.parse().ok()?;
    let hgt_unit = hgt.get(hgt.len() - 2..)?;
    if hgt_unit == "cm" && !(150 <= hgt_value && hgt_value <= 193) {
        return None;
    }
    if hgt_unit == "in" && !(59 <= hgt_value && hgt_value <= 76) {
        return None;
    }
    // Check hcl
    let hcl = fields.get("hcl")?;
    let hcl_head = hcl.get(..1)?;
    let hcl_tail = usize::from_str_radix(hcl.get(1..)?, 16).ok()?;
    if hcl_head != "#" || hcl_tail > 0xffffff {
        return None;
    }
    // Check ecl
    let ecl = fields.get("ecl")?;
    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl) {
        return None;
    }
    // Check pid
    let pid = fields.get("pid")?;
    if pid.len() != 9 || !pid.bytes().all(|x| b'0' <= x && x <= b'9') {
        return None;
    }
    Some(())
}

fn main() {
    let xs: Vec<String> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .collect::<Vec<_>>()
        .rsplit(|x| x == "")
        .map(|x| x.join(" "))
        .collect();
    let result = xs.iter().cloned().filter_map(|x| check_1(x)).count();
    println!("Part 1: {}", result);
    let result = xs.iter().cloned().filter_map(|x| check_2(x)).count();
    println!("Part 2: {}", result);
}
