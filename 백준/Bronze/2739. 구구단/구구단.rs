fn main() {
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        if let Ok(n) = input.trim().parse::<usize>() {
            for i in 1..=9 {
                println!("{} * {} = {}", n, i, n * i);
            }
        }
    }
}
