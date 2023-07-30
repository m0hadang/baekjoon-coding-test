use std::io;

fn main() {
    let mut inputs = String::new();
    match io::stdin().read_line(&mut inputs) {
        Ok(_) => {
            let numbers: Vec<&str> = inputs.split_whitespace().collect();
            let a_val = numbers[0].parse::<i32>().unwrap();
            let b_val = numbers[1].parse::<i32>().unwrap();
            println!("{}", a_val - b_val);
        }
        _ => {}
    }
}
