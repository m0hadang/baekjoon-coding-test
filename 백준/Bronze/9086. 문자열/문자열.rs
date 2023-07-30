fn main() {
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        if let Ok(n) = input.trim().parse::<usize>() {
            (0..n).for_each(|_|{
                input.clear();
                if let Ok(_) = std::io::stdin().read_line(&mut input) {
                    let input = input.trim();
                    println!("{}{}", 
                        &input.chars().nth(0).unwrap(), 
                        &input.chars().last().unwrap());
                }
            });
        }
    }
}