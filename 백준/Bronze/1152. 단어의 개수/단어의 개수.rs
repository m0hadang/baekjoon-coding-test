use std::io::{self};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("{}", input.trim().split_ascii_whitespace().count());
    Ok(())
}