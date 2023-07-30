use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut input: Vec<u8> = vec![];
    io::stdin().read_to_end(&mut input)?;

    const ALPA_LEN: usize = ('z' as usize) - ('a' as usize) + 1;
    let mut alpa_idx: [i32; ALPA_LEN] = [-1; ALPA_LEN];

    input
        .iter()
        .filter(|&&x| x.is_ascii_alphabetic())
        .enumerate()
        .for_each(|(i, &val)| {
        let idx: usize = (val as usize) - ('a' as usize);
        if alpa_idx[idx] == -1 {
            alpa_idx[idx] = i as i32;
        }
    });

    println!("{}", alpa_idx.map(|x| x.to_string()).join(" "));

    Ok(())
}
