fn main() {
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        if let Ok(n) = input.trim().parse::<usize>() {
            let long_str = (1..=(n / 4))
                .map(|_| "long")
                .collect::<Vec<&str>>()
                .join(" ");
            println!("{} int", long_str);
        }
    }
}