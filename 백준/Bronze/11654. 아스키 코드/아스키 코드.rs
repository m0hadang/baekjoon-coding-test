use std::io::Read;
fn main() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    std::io::stdin().read_to_end(&mut buffer)?;
    if let Some(ch) = buffer.first() {
        println!("{ch}");
    }
    Ok(())
}