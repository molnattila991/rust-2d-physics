
use sdl2::{render::Canvas, pixels::Color, rect::Point, video::Window};

pub trait Draw {
    fn draw_lines_with_color<'a, P>(&mut self, points: P, color: Color) -> Result<(), String> where P: Into<&'a [Point]>;
    fn draw_rectangle_with_color(&mut self, point: Point, width: i32, height: i32,  color: Color) -> Result<(), String>;
}

impl Draw for Canvas<Window> {
    fn draw_lines_with_color<'a, P>(&mut self, points: P, color: Color) -> Result<(), String> where P: Into<&'a [Point]> {
        self.set_draw_color(color);

        let result = self.draw_lines(&points.into()[..]);

        Ok(())
    }

    fn draw_rectangle_with_color(&mut self, point: Point, width: i32, height: i32, color: Color) -> Result<(), String> {
        self.set_draw_color(color);

        let points: [Point; 5] = [
            Point::new(point.x, point.y), 
            Point::new(point.x + width, point.y), 
            Point::new(point.x + width, point.y + height), 
            Point::new(point.x, point.y + height),
            Point::new(point.x, point.y)
            ];
        
        let result = self.draw_lines(&points[..]);

        Ok(())
    }
}
