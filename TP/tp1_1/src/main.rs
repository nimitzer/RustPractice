mod base_types;
mod rectangle;
mod circle;
mod ellipse;
mod shape;

use crate::shape::{Shape};
use crate::base_types::{Point};
use crate::circle::{Circle};
use crate::ellipse::{Ellipse};
use crate::rectangle::{Rectangle};

use std::io::{self, BufRead};
use std::process;

fn main() {
    let stdin = io::stdin();

    // let file = File::open("input.txt").unwrap_or_else(|err| {
    //     eprintln!("Ошибка при открытии файла: {}", err);
    //     process::exit(1);
    // });
    // let reader = BufReader::new(file);

    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    let mut was_scale = false;
    let mut scale_point = Point{x: 0.0, y: 0.0};
    let mut scale_k = 1.0;

    'main_cycle: for raw_line in stdin.lock().lines() {
        if let Ok(line) = raw_line {
            let mut parts = line.split_whitespace();
            
            if let Some(description) = parts.next() {
                let mut vector = Vec::new();
                for maybe in parts {
                    match maybe.parse::<f64>(){
                        Ok(num) => vector.push(num),
                        Err(_) => continue 'main_cycle,
                    };
                }
                
                match description {
                    "RECTANGLE" => {
                        if vector[0] > vector[2] || vector[1] > vector[3] {
                            eprintln!("Неверные данные: Rectangle");
                            continue;
                        }

                        shapes.push(Box::new(Rectangle {
                            bottom_left_corner: Point {x: vector[0], y: vector[1]},
                            top_right_corner: Point {x: vector[2], y: vector[3]},
                        }));
                    }

                    "CIRCLE" => {
                        if vector[2] <= 0.0 {
                            eprintln!("Неверные данные: Circle");
                            continue;
                        }

                        shapes.push(Box::new(Circle{
                            pos: Point{x: vector[0], y: vector[1]},
                            radius: vector[2],
                        }));
                    }

                    "ELLIPSE" => {
                        if vector[2] <= 0.0 || vector[3] <= 0.0 {
                            eprintln!("Неверные данные: Ellipse");
                            continue; 
                        }

                        shapes.push(Box::new(Ellipse{
                            pos: Point{x: vector[0], y: vector[1]},
                            radius_y: vector[2],
                            radius_x: vector[3], 
                        }));
                    }

                    "SCALE" => {
                        if vector[2] <= 0.0 {
                            eprintln!("Некорректный коэффициент масштабирования");
                            process::exit(1);
                        }
                        was_scale = true;
                        scale_point = Point{x: vector[0], y: vector[1]};
                        scale_k = vector[2];
                    }

                    _ => { 
                        continue; 
                    }
                }
            }
        }
    }

    if was_scale == false {
        eprintln!("Не введена команда масштабирования");
        process::exit(1);
    }

    print!("{:.1} ", shapes.iter().map(|s| s.get_area()).sum::<f64>());

    shapes.iter_mut()
        .for_each(|s| {
            let rec = s.get_frame_rect();
            print!("{:.1} {:.1} {:.1} {:.1} ", 
                rec.bottom_left_corner.x,
                rec.bottom_left_corner.y,
                rec.top_right_corner.x,
                rec.top_right_corner.y);
            s.scale(&scale_k, &scale_point);
        });

    println!();

    print!("{:.1} ", shapes.iter().map(|s| s.get_area()).sum::<f64>());

    shapes.iter()
        .for_each(|s| {
            let rec = s.get_frame_rect();
            print!("{:.1} {:.1} {:.1} {:.1} ", 
                rec.bottom_left_corner.x,
                rec.bottom_left_corner.y,
                rec.top_right_corner.x,
                rec.top_right_corner.y);
        });
}