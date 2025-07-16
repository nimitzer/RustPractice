use std::str::FromStr;
use std::process;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Заглушка возвращаемой ошибки, можно оставить пустым
pub struct PointParseError;

// Реализация трейта FromStr, нужен для чтения точки из потока ввода 
impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if !s.starts_with('(') || !s.ends_with(')') {
            return Err(PointParseError);
        }

        let content = &s[1..s.len()-1];
        let parts: Vec<&str> = content.split(';').collect();

        let x = parts[0].trim().parse::<f64>().map_err(|_| PointParseError)?;
        let y = parts[1].trim().parse::<f64>().map_err(|_| PointParseError)?;

        Ok(Point{x, y})
    }
}

// Тип данных для прямой линии
#[derive(Debug, Clone)]
pub struct StraightLine<'a> {
    pub coords: (&'a Point, &'a Point),
}

// Проверка на вырожденность (все на одной прямой, через векторное умножение)
pub fn is_degenerate(lines: &Vec<StraightLine>) -> bool {
    if lines.len() <= 1 {
        return true;
    }

    let (p0, p1) = lines[0].coords;

    lines.iter()
        .skip(1)
        .all(|line| {
            let (a, b) = line.coords;
            (p1.x - p0.x) * (b.y - a.y) - 
            (p1.y - p0.y) * (b.x - a.x) <= f64::EPSILON
        })
}

// Длина ломанной линии ()
fn get_length(lines: &Vec<StraightLine>) -> f64 {
    if lines.len() < 1 {
        return 0.0;
    }

    lines.iter()
        .map(|line| {
            let (a, b) = line.coords;
            ((a.x - b.x).powi(2) + 
            (a.y - b.y).powi(2))
            .sqrt()
        })
        .sum()
}

fn main() {
    // Заключаем последовательность точек в вектор
    let line = io::stdin().lock().lines().next().unwrap().unwrap();

    let points: Vec<Point> = line
        .split_whitespace()
        .map(|s| s.parse::<Point>()
        .unwrap_or_else(|_| process::exit(1)))
        .collect();

    if points.len() < 2 {
        process::exit(1)
    }

    // Вектор прямых
    let straights: Vec<StraightLine> = points
        .iter()
        .zip(points.iter().cycle().skip(1).take(points.len() - 1 ))
        .map(|(p1, p2)| StraightLine{coords: (p1, p2)})
        .collect();

    println!("{}", if is_degenerate(&straights) { 1 } else { 0 });
    println!("{:.2}", get_length(&straights));
}
