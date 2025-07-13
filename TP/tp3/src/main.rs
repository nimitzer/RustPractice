mod polygon;
mod commands;

use std::env;
use std::process;
use std::fs;
use std::io::{self, BufRead};

use crate::polygon::Polygon;
use crate::commands::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Неправильный ввод имени файла");
        process::exit(1);
    }

    let input = &args[1];
    let file = fs::read_to_string(format!("{}.txt", input))
        .unwrap_or_else(|_| {
            eprintln!("Ошибка чтения файла");
            process::exit(1);
        });

    let polygons: Vec<_> = file.lines()
        .filter_map(|s| s.parse::<Polygon>().ok())
        .collect();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap_or_default();
        let mut parts = line.splitn(2, ' ');
        match parts.next() {
            Some("AREA")     => handle_area(&polygons, parts.next()),
            Some("MAX")      => handle_max(&polygons, parts.next()),
            Some("MIN")      => handle_min(&polygons, parts.next()),
            Some("COUNT")    => handle_count(&polygons, parts.next()),
            Some("MAXSEQ")   => handle_maxseq(&polygons, parts.next()),
            Some("LESSAREA") => handle_lessarea(&polygons, parts.next()),
            Some(_)          => println!("<INVALID COMMAND>"),
            None             => {}
        }
    }
}