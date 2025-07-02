use std::env;
use std::process;
use std::fs;
use std::fs::File;
use std::io::Write;

fn f(matrix: Vec<Vec<i32>>, rows: usize, columns: usize) -> usize {
    let mut result: usize = 0;
    for i in 1..rows-1 {
        for j in 1..columns-1 {
            let number = matrix[i][j];
            

            let mut is_local_max = true;
            for k in i-1..i+2 {
                for q in j-1..j+2 {
                    if (k == i) && (q == j) {
                        continue;
                    }
                    if number <= matrix[k][q] {
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

fn main() -> std::io::Result<()> {
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

    let content = file.split_whitespace();
    
    let vec = file.split_whitespace()
        .map(|x| x.parse::<i32>()
        .unwrap_or_else(|_| process::exit(2)))
        .collect::<Vec<i32>>();

    if vec.len() < 2 {
        process::exit(2)
    }

    let (rows, columns) = (vec[0] as usize, vec[1] as usize);

    if rows * columns != vec.len() - 2 {
        process::exit(1);
    }

    if num == 1 {
        if rows == 0 || columns == 0 {
            writeln!(out, "0");
            return Ok(())
        }

        let matrix: Vec<Vec<i32>> = vec[2..].chunks(columns)
            .map(|chunk| chunk.to_vec())
            .collect();
        
        writeln!(out, "{}", f(matrix, rows, columns).to_string());
    }
    
    else {
        let mut matrix: Vec<Vec<i32>> = vec[2..].chunks(columns)
            .map(|chunk| chunk.to_vec())
            .collect();

        writeln!(out, "{}", f(matrix, rows, columns).to_string());
    }
    Ok(())
}