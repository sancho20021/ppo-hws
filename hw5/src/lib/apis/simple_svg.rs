use std::fs;

use simplesvg::{Attr, ColorAttr, Fig, Svg};

use crate::drawing_api::DrawingApi;

pub struct SimpleSvg {
    width: u32,
    height: u32,
    figs: Vec<Fig>,
}

impl SimpleSvg {
    pub fn new() -> Self {
        SimpleSvg {
            width: 1080,
            height: 1080,
            figs: vec![],
        }
    }
}

impl DrawingApi for SimpleSvg {
    fn get_area_width(&self) -> u32 {
        self.width
    }

    fn get_area_height(&self) -> u32 {
        self.height
    }

    fn draw_line(&mut self, from: &crate::drawing_api::Point, to: &crate::drawing_api::Point) {
        let fig = Fig::Line(from.x as f32, from.y as f32, to.x as f32, to.y as f32);

        let color = ColorAttr::Color(0, 255, 255);
        let style = Attr::default().stroke_width(5.0).stroke(color);

        let fig = Fig::Styled(style, Box::new(fig));
        self.figs.push(fig);
    }

    fn draw_circle(&mut self, position: &crate::drawing_api::Point, radius: u32) {
        let fig = Fig::Circle(position.x as f32, position.y as f32, radius as f32);
        self.figs.push(fig);
    }

    fn export_svg(&self, file: &String) {
        let output = Svg(self.figs.clone(), self.width, self.height).to_string();
        fs::write(file, output).expect("Unable to write file");
    }
}
