use std::io;

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let mut sp = input.split_ascii_whitespace();
        if let (Some(n), Some(m)) = (sp.next(), sp.next()) {
            if let (Ok(n), Ok(m)) = (n.parse::<usize>(), m.parse::<u32>()) {
                let mut arr: Vec<u32> = vec![0; n];
                (0..n).for_each(|x| arr[x] = (x + 1) as u32);
                for _ in 0..m {
                    input.clear();
                    if let Ok(_) = io::stdin().read_line(&mut input) {
                        let mut sp = input.split_ascii_whitespace();
                        if let (Some(i), Some(j)) = (sp.next(), sp.next()) {
                            if let (Ok(i), Ok(j)) = (i.parse::<usize>(), j.parse::<usize>()) {
                                arr.swap(i - 1, j - 1);
                            }
                        }
                    }
                }

                println!("{}", arr.iter().map(ToString::to_string).collect::<Vec<String>>().join(" "));
            }
        }
    }
}