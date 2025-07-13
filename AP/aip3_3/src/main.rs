use std::env;
use std::process;
use std::fs;
use std::io::Write;

const MAX_SIZE: usize = 10000;

enum ArrayOrBoxed<T, const N: usize> {
    Stack([T; N]),
    Heap(Box<[T; N]>),
}

fn f(matrix: ArrayOrBoxed<i32, MAX_SIZE>, rows: usize, columns: usize) -> usize {
    let arr_ref = match &matrix {
        ArrayOrBoxed::Stack(arr) => arr.as_slice(),
        ArrayOrBoxed::Heap(boxed_arr) => boxed_arr.as_slice(),
    };

    let mut result: usize = 0;

    for i in 1..rows-1 {
        for j in 1..columns-1 {
            let number = arr_ref[i*columns + j];
            
            let mut is_local_max = true;
            for k in i-1..i+2 {
                for q in j-1..j+2 {
                    if (k == i) && (q == j) {
                        continue;
                    }
                    if number <= arr_ref[k*columns + q] {
                        is_local_max = false;
                    }
                }
            }

            if is_local_max == true {
                result += 1;
            }
        }
    }
    result
}

fn main(){
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 4 {
        process::exit(1);
    }

    let num = match args[1].parse::<u8>() {
        Ok(a) =>  if a == 1 || a == 2 { a } else { process::exit(1) },
        _ => process::exit(1),
    };

    let (input, output) = (&args[2], &args[3]);
    
    let file = fs::read_to_string(format!("{input}.txt")).unwrap_or_else(|_| process::exit(1));
    let mut out = fs::File::create(format!("{output}.txt")).unwrap_or_else(|_| process::exit(1));

    let mut arr: [i32; MAX_SIZE] =  [0; MAX_SIZE];

    let mut iter = file.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap_or_else(|_|  { 
            eprintln!("Невозможно представить как последовательность");
            process::exit(2);
}       ))
        .take(MAX_SIZE);

    let rows_columns = (
        iter.next().map(|x| (x as usize)).unwrap_or(0),
        iter.next().map(|x| (x as usize)).unwrap_or(0),
    );

    for (i, num) in iter.enumerate() {
        if i < MAX_SIZE - 2 {
            arr[i] = num;
        }
    }

    if arr.len() < 2 {
        eprintln!("Длина последовательности неверна");
        process::exit(2);
    }

    let result: ArrayOrBoxed<i32, MAX_SIZE> = match num {
        1 => ArrayOrBoxed::Stack(arr),
        2 => ArrayOrBoxed::Heap(Box::new(arr)),
        _ => process::exit(1),
    };

    writeln!(out, "{}", f(result, rows_columns.0, rows_columns.1).to_string())
        .unwrap_or_else(|_| process::exit(1));
}