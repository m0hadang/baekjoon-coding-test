fn main() {
    let mut str_input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut str_input) {
        let mut num_input = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut num_input) {
            if let Ok(i) = num_input.trim().parse::<usize>() {
                println!("{}", str_input.chars().nth(i - 1).unwrap());
            }
        }        
    }
}