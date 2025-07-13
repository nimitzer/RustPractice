use crate::base_types::{Point};
use crate::shape::Shape;
use crate::rectangle::Rectangle;

use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub radius_y : f64,
    pub radius_x : f64,
    pub pos: Point,
}

impl Shape for Ellipse {
    fn get_area(&self) -> f64 {
        self.radius_y * self.radius_x * PI
    }

    fn get_frame_rect(&self) -> Rectangle {
        Rectangle {
            bottom_left_corner: Point {
                x: self.pos.x - self.radius_x,
                y: self.pos.y - self.radius_y,
            },
            top_right_corner: Point {
                x: self.pos.x + self.radius_x,
                y: self.pos.y + self.radius_y,
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
        self.radius_x *= k;
        self.radius_y *= k;

        self.pos.x = scale_center.x + 
            k * (self.pos.x - scale_center.x);

        self.pos.y = scale_center.y + 
            k * (self.pos.y - scale_center.y);
    }
}