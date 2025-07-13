use std::io;
use std::process;

fn f(input: String) -> usize {
    input
        .chars()
        .filter(|c| ('a'..='z').contains(c) || ('A'..='Z').contains(c))
        .count()
}

fn main() {
    let mut data = String::new();

    if io::stdin().read_line(&mut data).is_err() {
        process::exit(1);
    }

    let result = f(data);

    println!("{}", result);
}