use std::io;
use std::process;

fn f(mut first: String) -> String {
    let second: String = String::from("example");

    first.retain(|c| (second.contains(c)));
    first
}

fn main() {
    let mut data = String::new();

    if io::stdin().read_line(&mut data).is_err() {
        process::exit(1);
    }

    let result = f(data);

    println!("{}", result);
}