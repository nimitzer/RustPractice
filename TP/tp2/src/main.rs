mod data_struct;

use std::io::{self, BufRead};
use std::process;
use std::cmp::Ordering;

use std::fs::File;
use std::io::BufReader;

use crate::data_struct::{DataStruct};

fn main() {
    let stdin = io::stdin();
    // let file = File::open("input.txt").unwrap_or_else(|err| {
    //     eprintln!("Ошибка при открытии файла: {}", err);
    //     process::exit(1);
    // });
    // let reader = BufReader::new(file);

    let mut data_vector: Vec<DataStruct> = stdin
        .lines()
        .filter_map(|line_result| {
            line_result.ok().and_then(|line| line.parse::<DataStruct>().ok())
        })
        .collect();

    data_vector.sort_by(|left, right| {
        match left.key1.partial_cmp(&right.key1).unwrap_or(Ordering::Equal) {
            Ordering::Equal => match left.key2.partial_cmp(&right.key2).unwrap_or(Ordering::Equal) {
                Ordering::Equal => left.key3.len().cmp(&right.key3.len()),
                other => other,
            },
            other => other,
        }
    });

    data_vector.iter().for_each(|d| println!("{}", d));
}
