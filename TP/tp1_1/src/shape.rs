use crate::base_types::{Point};
use crate::circle::Circle;
use crate::ellipse::Ellipse;
use crate::rectangle::Rectangle;

pub trait Shape {
    fn get_area(&self) -> f64;
    fn get_frame_rect(&self) -> Rectangle;
    fn moved(&mut self, point: &Point);
    fn moved_by(&mut self, shift_x: f64, shift_y: f64);
    fn scale(&mut self, k: &f64, scale_center: &Point);
}