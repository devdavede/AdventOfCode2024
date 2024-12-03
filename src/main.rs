use std::io::{self};

pub mod implementations {
    pub mod day1;
    pub mod day2;
    pub mod hlpr;
}

fn main() -> io::Result<()> {
    implementations::day2::run()?;
    Ok(())
}