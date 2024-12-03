use std::io::{self};

pub mod implementations {
    pub mod day1;
}

fn main() -> io::Result<()> {
    implementations::day1::run()?;
    Ok(())
}