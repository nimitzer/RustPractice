use std::io;
use std::process;

fn f(input: String) -> String {
    input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    let mut data = String::new();

    if io::stdin().read_line(&mut data).is_err() {
        process::exit(1);
    }

    let result = f(data);

    println!("{}", result);
}