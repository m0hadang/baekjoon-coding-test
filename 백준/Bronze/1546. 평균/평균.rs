
fn main() {
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        if let Ok(m) = input.trim().parse::<u32>() {
            input.clear();
            if let Ok(_) = std::io::stdin().read_line(&mut input) {
                let vec = input
                    .trim()
                    .split_ascii_whitespace()
                    .filter_map(|x| x.parse::<u32>().ok())
                    .collect::<Vec<u32>>();
                let& max_val = vec.iter().max().unwrap();
                let max_val = max_val as f32;
                let sum = vec
                    .iter()
                    .map(|&x| x as f32)
                    .map(|x| (x / max_val)  * 100.0).sum::<f32>();
                println!("{}", sum / m as f32);
            }
        }
    }
}