use std::io;
use std::process;

fn f(x: &f64, k: &u32, error: &f64) -> Option<f64> {
    let mut n: u32 = 0;
    let mut result: f64 = 0.0;
    let mut term: f64 = *x;

    while n <= *k {
        if term.abs() < *error {
            return Some(result);
        }
        result += term;

        n += 1;

        term *= -(*x) * (*x) * (2.0 * (n as f64) - 1.0) / (2.0 * (n as f64) + 1.0);
    }

    None
}

fn main() {
    let mut data = String::new();
    let range: (f64, f64) = (-1.0, 1.0);
    let error: f64 = 0.000001;
    let step: f64 = 0.1;

    if io::stdin().read_line(&mut data).is_err() {
        process::exit(1);
    }

    let parts: Vec<&str> = data.trim().split_whitespace().collect();
    
    if parts.len() != 3 {
        process::exit(1);
    }

    let (start, end, quantity) = match (
        parts[0].parse::<f64>(),
        parts[1].parse::<f64>(),
        parts[2].parse::<u32>(),
    ) {
        (Ok(a), Ok(b), Ok(c)) => (a, b, c),
        _ => process::exit(1),
    };

    if start >= end {
        process::exit(1);
    }

    if !(start > range.0 && end < range.1) {
        process::exit(1);
    }

    let mut next = start;

    while next <= end {
        print!("|{:^12.8}|", next);
        print!(" ");
        let stdfunc = next.atan();
        print!("|{:^12.8}|", stdfunc);
        let myfunc = f(&next, &quantity, &error);
        match myfunc {
            Some(n) => print!("|{:^12.8}|", n),
            None => print!("<|{:^12}|>", "MATH_ERROR"),
        }
        
        println!();
        next += step;
    }
}