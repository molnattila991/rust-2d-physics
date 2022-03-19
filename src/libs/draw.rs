
use sdl2::{render::Canvas, pixels::Color, rect::Point, video::Window};

pub trait Draw {
    fn draw_line_with_color<'a, P>(&mut self, points: P, color: Color) -> Result<(), String> where P: Into<&'a [Point]>;
}

impl Draw for Canvas<Window> {
    fn draw_line_with_color<'a, P>(&mut self, points: P, color: Color) -> Result<(), String> where P: Into<&'a [Point]> {
        self.set_draw_color(color);

        self.draw_lines(&points.into()[..]);

        Ok(())
    }
}
