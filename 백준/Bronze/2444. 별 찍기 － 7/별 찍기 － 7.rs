use std::io::{self};

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        if let Ok(n) = input.trim().parse::<usize>() {
            let max_stars_width = 1 + ((n - 1) * 2);

            let mut v: Vec<String> = Vec::with_capacity(n * 2);
            let mut stars_width = 1;
            (0 .. n - 1).for_each(|_| {
                let stars: String = (0..stars_width).map(|_| '*').collect();
                let stars = format!("{:^width$}", stars, width = max_stars_width);
                let stars = stars.trim_end().to_string();
                v.push(stars);
                stars_width += 2;
            });

            for item in v.iter() {
                println!("{item}");
            }

            let stars: String = (0..max_stars_width).map(|_| '*').collect();
            println!("{stars}");
            
            for item in v.iter().rev() {
                println!("{item}");
            }
        }
    }
}