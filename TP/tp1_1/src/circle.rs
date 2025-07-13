use crate::base_types::{Point};
use crate::shape::Shape;
use crate::rectangle::Rectangle;

use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Circle {
    pub radius : f64,
    pub pos: Point,
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        self.radius * self.radius * PI
    }

    fn get_frame_rect(&self) -> Rectangle {
        Rectangle {
            bottom_left_corner: Point {
                x: self.pos.x - self.radius,
                y: self.pos.y - self.radius,
            },
            top_right_corner: Point {
                x: self.pos.x + self.radius,
                y: self.pos.y + self.radius,
            },
        }
    }

    fn moved(&mut self, point: &Point) {
        self.pos = point.clone();
    }

    fn moved_by(&mut self, dx: f64, dy : f64) {
        self.pos.x += dx;
        self.pos.y += dy;
    }

    fn scale(&mut self, k: &f64, scale_center: &Point) {
        self.radius *= k;

        self.pos.x = scale_center.x + 
            k * (self.pos.x - scale_center.x);

        self.pos.y = scale_center.y + 
            k * (self.pos.y - scale_center.y);
    }
}