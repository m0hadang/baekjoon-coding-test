use std::{io::{Read}, collections::HashSet};

fn main() {
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_to_string(&mut input) {
        let mut s: HashSet<u32> = HashSet::new();
        input
            .trim()
            .split('\n')
            .filter_map(|x| x.parse::<u32>().ok())
            .map(|x| x % 42)
            .for_each(|x| {
                if !s.contains(&x) {
                    s.insert(x);
                }
            });
        println!("{}", s.len());
    }
}