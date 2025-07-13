use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub struct PointParseError;

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({};{})", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if !s.starts_with('(') || !s.ends_with(')') {
            return Err(PointParseError);
        }

        let content = &s[1..s.len()-1];
        let parts: Vec<&str> = content.split(';').collect();

        let x = parts[0].trim().parse::<i32>().map_err(|_| PointParseError)?;
        let y = parts[1].trim().parse::<i32>().map_err(|_| PointParseError)?;

        Ok(Point{x, y})
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub points: Vec<Point>,
}

pub struct PolygonParseError;

impl Polygon {
    pub fn area(&self) -> f64 {
        let n = self.points.len();
        if n < 3 {
            return 0.0;
        }

        let sum: f64 = self.points
            .iter()
            .zip(self.points.iter().cycle().skip(1).take(n))
            .map(|(p1, p2)| (p1.x * p2.y - p2.x * p1.y) as f64)
            .sum();

        (sum / 2.0).abs()
    }
}

impl Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.points.len())?;
        for point in &self.points {
            write!(f, "{} ", point)?;
        }
        Ok(())
    }
}

impl FromStr for Polygon {
    type Err = PolygonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let count_str = parts.next().ok_or(PolygonParseError)?;
        let count: usize = count_str.parse().map_err(|_| PolygonParseError)?;

        let mut points = Vec::with_capacity(count);
        for maybe_point in parts {
            let point = maybe_point.parse::<Point>().map_err(|_| PolygonParseError)?;
            points.push(point);
        }

        if points.len() != count {
            return Err(PolygonParseError);
        }

        if points.len() < 3 {
            return Err(PolygonParseError);
        }

        Ok(Polygon { points })
    }
}