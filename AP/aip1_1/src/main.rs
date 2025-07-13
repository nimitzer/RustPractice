use std::io::{self, BufRead};
use std::process;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut current: Option<i32> = None;
    let mut result: u32 = 0;

    'mark : for line_result in lines {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => {
                eprintln!("Невозможно интерпретировать как последовательность");
                process::exit(1);
            },
        };

        for token in line.split_whitespace() {
            let new = match token.parse::<i32>() {
                Ok(new) => new,
                Err(_) => {
                    eprintln!("Невозможно интерпретировать как последовательность");
                    process::exit(1);
                },
            };

            if new == 0 {
                break 'mark;
            }

            if current.is_some() {
                if new > current.unwrap_or(0){
                    result += 1;
                }
            }

            current = Some(new);
        }
    }

    println!("{}", result);
}