use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_to_string(&mut input) {
        let mut arr = [false; 30];
        let vec = input.trim().split('\n').filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        for &num in vec.iter() {
            // println!("-> {}", num);
            arr[num - 1] = true;
        }
        for (idx, &flag) in arr.iter().enumerate() {
            if !flag {
                println!("{}", idx + 1);
            }
        }
    }
}