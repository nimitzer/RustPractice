use std::io::{self, BufRead};
use std::process;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut current: Option<i32> = None;
    let mut result: u32 = 0;
    let mut max: i32 = i32::MIN;

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

            if new > max {
                max = new;
                result = 1;
            }
            else if new == max {
                result += 1;
            }

            current = Some(new);
        }
    }

    if current.is_none() {
        eprintln!("Максимального числа не существует");
        process::exit(2);
    }

    println!("{}", result);
}