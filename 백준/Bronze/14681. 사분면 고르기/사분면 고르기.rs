use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    if let (Ok(_), Ok(_)) = (
        io::stdin().read_line(&mut input1),
        io::stdin().read_line(&mut input2),
    ) {
        if let (Ok(x), Ok(y)) = (input1.trim().parse::<i32>(), input2.trim().parse::<i32>()) {
            let ans = match (x, y) {
                (x, y) if x > 0 && y > 0 => 1,
                (x, y) if x < 0 && y > 0 => 2,
                (x, y) if x < 0 && y < 0 => 3,
                (x, y) if x > 0 && y < 0 => 4,
                _ => 0,
            };
            println!("{}", ans);
        } else {
            println!("Error: Not a number");
        }
    }
}