use std::fs::File;
use std::io::{self, BufRead};

/*
    The levels are either all increasing or all decreasing.
    Any two adjacent levels differ by at least one and at most three.
*/

pub fn run() -> io::Result<()> {
    let file = File::open("./src/implementations/day2_input.txt")?;
    let reader = io::BufReader::new(file);
    let mut safe: i32 = 0;

    for line in reader.lines() {
        let mut skip = false;
        let line = line?;
        let parts: Vec<&str> = line.split(" ").collect();
        let mut direction = -1;

        let first_item: i32 = parts[0].parse().unwrap();
        let second_item: i32 = parts[1].parse().unwrap();
        direction = if first_item < second_item { 1 } else { 0 };

        for i in 0..parts.len() - 1 {
            let curr: i32 = parts[i].parse().unwrap();
            let nxt: i32 = parts[i + 1].parse().unwrap();

            if (curr == nxt
                || (curr < nxt && direction == 0)
                || (curr > nxt && direction == 1)
                || (curr - nxt == 0)
                || ((curr - nxt).abs() > 3))
            {
                skip = true;
                println!("Unsafe: {}", line);
                break;
            }
        }

        if (!skip) {
            safe = safe + 1;
            println!("Safe: {}", line)
        }
    }

    println!("Cnt Safe level: {}", safe);
    Ok(())
}
