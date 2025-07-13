use std::io::{self, BufRead};
use std::process;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut current: Option<i32> = None;
    let mut result: u32 = 0;
    let mut is_lesser: bool = false;

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
            if is_lesser == true {
                if current > Some(new) {
                    result += 1;
                    is_lesser = false;
                }
            }

            if current.is_some() {
                if current > Some(new) {
                    is_lesser = true;
                }
            }

            current = Some(new);
        }
    }

    println!("{}", result);
}