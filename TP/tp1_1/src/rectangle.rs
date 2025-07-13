use crate::base_types::{Point};
use crate::shape::Shape;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub bottom_left_corner: Point,
    pub top_right_corner: Point,
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        (self.top_right_corner.x - self.bottom_left_corner.x).abs() *
        (self.top_right_corner.y - self.bottom_left_corner.y).abs()
    }

    fn get_frame_rect(&self) -> Rectangle {
        self.clone()
        // rectangle_t {
        //     width : (self.top_right_corner.x - self.bottom_left_corner.x).abs(),
        //     height : (self.top_right_corner.y - self.bottom_left_corner.y).abs(),
        //     pos : Point {
        //         x: (self.top_right_corner.x + self.bottom_left_corner.x) / 2.0,
        //         y: (self.top_right_corner.y + self.bottom_left_corner.y) / 2.0,
        //     }
        // }
    }

    fn moved(&mut self, point: &Point) {
        let demiwidth = (self.top_right_corner.x - self.bottom_left_corner.x)
            .abs() / 2.0;
        let demiheight = (self.top_right_corner.y - self.bottom_left_corner.y)
            .abs() / 2.0;

        self.bottom_left_corner.x = point.x + demiwidth;
        self.bottom_left_corner.y = point.y + demiheight;

        self.top_right_corner.x = point.x + demiwidth;
        self.top_right_corner.y = point.y + demiheight;
    }

    fn moved_by(&mut self, dx: f64, dy : f64) {
        self.bottom_left_corner.x += dx;
        self.bottom_left_corner.y += dy;

        self.top_right_corner.x += dx;
        self.top_right_corner.y += dy;
    }

    fn scale(&mut self, k: &f64, scale_center: &Point) {
        self.bottom_left_corner.x = scale_center.x + 
            k * (self.bottom_left_corner.x - scale_center.x);

        self.bottom_left_corner.y = scale_center.y + 
            k * (self.bottom_left_corner.y - scale_center.y);

        self.top_right_corner.x = scale_center.x + 
            k * (self.top_right_corner.x - scale_center.x);

        self.top_right_corner.y = scale_center.y + 
            k * (self.top_right_corner.y - scale_center.y);
    }
}