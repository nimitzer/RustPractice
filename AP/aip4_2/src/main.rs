use std::io;
use std::process;

fn f(mut input: String) -> String {
    input.retain(|c| !("0123456789".contains(c)))
    input
}

fn main() {
    let mut data = String::new();

    if io::stdin().read_line(&mut data).is_err() {
        process::exit(1);
    }

    let result = f(data);

    println!("{}", result);
}