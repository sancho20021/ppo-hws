use crate::apis::{draw::Draw, simple_svg::SimpleSvg};

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub trait DrawingApi {
    fn get_area_width(&self) -> u32;
    fn get_area_height(&self) -> u32;
    fn draw_line(&mut self, from: &Point, to: &Point);
    fn draw_circle(&mut self, position: &Point, radius: u32);
    /// panics on failure
    fn export_svg(&self, file: &String);
}

/// scales point inside -1:1 square to given area
pub fn scale(x: f32, y: f32, x_limits: &(u32, u32), y_limits: &(u32, u32)) -> Point {
    let transfer = |a: f32, left: u32, right: u32| -> u32 {
        ((a + 1.0) * (right - left) as f32 / 2.0 + left as f32) as u32
    };

    Point {
        x: transfer(x, x_limits.0, x_limits.1),
        y: transfer(y, y_limits.0, y_limits.1),
    }
}
