use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("   ").collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    let mut fulldiff = 0;
    for i in 0..left.len() { // Range is 1 to 5 (end is inclusive)
        let diff = (left[i] - right[i]).abs();
        fulldiff += diff;
    }

    println!("The difference is: {}", fulldiff);
    Ok(())
}