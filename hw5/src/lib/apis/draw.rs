use draw::{render, Canvas, Color, Drawing, LineBuilder, Shape, Style, SvgRenderer};

use crate::drawing_api::DrawingApi;

pub struct Draw {
    canvas: Canvas,
}

impl Draw {
    pub fn new() -> Self {
        Draw {
            canvas: Canvas::new(1080, 1080),
        }
    }
}

impl DrawingApi for Draw {
    fn get_area_width(&self) -> u32 {
        self.canvas.width
    }

    fn get_area_height(&self) -> u32 {
        self.canvas.height
    }

    fn draw_line(&mut self, from: &crate::drawing_api::Point, to: &crate::drawing_api::Point) {
        let line = Drawing::new()
            .with_shape(
                LineBuilder::new(from.x as f32, from.y as f32)
                    .line_to(to.x as f32, to.y as f32)
                    .build(),
            )
            .with_style(Style::stroked(5, Color::gray(50)));

        self.canvas.display_list.add(line);
    }

    fn draw_circle(&mut self, position: &crate::drawing_api::Point, radius: u32) {
        let circle = Drawing::new()
            .with_shape(Shape::Circle { radius })
            .with_xy(position.x as f32, position.y as f32)
            .with_style(Style::filled(Color::black()));

        self.canvas.display_list.add(circle);
    }

    fn export_svg(&self, file: &String) {
        // save the canvas as an svg
        render::save(&self.canvas, file, SvgRenderer::new()).expect("Failed to save");
    }
}
