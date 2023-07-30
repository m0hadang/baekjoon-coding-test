use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if let Ok(n) = input.trim().parse::<usize>() {
        let mut buffer: Vec<u8> = vec![0; n];
        io::stdin().read_exact(&mut buffer)?;
        println!("{}", 
            buffer
            .iter()
            .filter_map(|&x| char::from(x).to_digit(10))
            .sum::<u32>());
    }
    Ok(())
}