use std::io;

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        if let Ok(num) = input.trim().parse::<u32>() {
            let grade = match num {
                60..=69 => "D",
                70..=79 => "C",
                80..=89 => "B",
                90..=100 => "A",
                _ => "F",
            };
            println!("{}", grade);
        } else {
            println!("Error: Not a number");
        }
    }
}
